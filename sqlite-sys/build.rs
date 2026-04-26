//! Build script for the `sqlite` C api

use std::fs::File;
use std::io::Write;
use std::process::Command;

struct SQLiteBuilder {
    build_dir: std::path::PathBuf,
    configure_path: std::path::PathBuf,
    sqlite_3_h_path: std::path::PathBuf,
    bindgen_out_path: std::path::PathBuf,
}

impl SQLiteBuilder {
    const BUILD_DIR: &'static str = "sqlite3_build";

    const SOURCE_DIR: &'static str = "sqlite_c_src";

    fn new() -> Self {
        let current_dir = std::env::current_dir().expect("unable to get current directory");
        let build_dir = current_dir.join(Self::BUILD_DIR);
        let script_path = current_dir.join("build.rs");
        let configure_path = current_dir.join(Self::SOURCE_DIR).join("configure");
        let sqlite_3_h_path = current_dir.join(Self::BUILD_DIR).join("sqlite3.h");
        let bindgen_out_path = current_dir.join("src").join("sqlite_c_api.rs");

        println!("cargo:rerun-if-changed={}", script_path.display());
        println!("cargo:rerun-if-changed={}", configure_path.display());
        println!("cargo:rerun-if-changed={}", sqlite_3_h_path.display());

        Self {
            build_dir,
            configure_path,
            sqlite_3_h_path,
            bindgen_out_path,
        }
    }

    fn build(&self) {
        self.create_build_dir();
        self.configure_build();
        Command::new("make")
            .arg("libsqlite3.a")
            .current_dir(&self.build_dir)
            .output()
            .expect("unable to build libsqlite3.a");
    }

    fn create_build_dir(&self) {
        if !self.build_dir.exists() {
            std::fs::create_dir(&self.build_dir).expect("failed to create build dir");
        }
    }

    fn configure_build(&self) {
        Command::new(&self.configure_path)
            .current_dir(&self.build_dir)
            .output()
            .expect("failed to run configure");
    }

    fn bindgen(&self) {
        let bindgens = bindgen::Builder::default()
            .header(self.sqlite_3_h_path.display().to_string())
            .generate()
            .expect("bindgen failed");

        let mut file_bindgen = File::create(&self.bindgen_out_path).unwrap();

        Self::add_allowed_lints_for_bindgen(&file_bindgen);
        bindgens.write(Box::new(&mut file_bindgen)).unwrap();
    }

    fn add_allowed_lints_for_bindgen(mut file: &File) {
        const ALLOWED_LINTS: &[&str] = &[
            "clippy::unreadable_literal",
            "unused_parens",
            "non_camel_case_types",
            "non_snake_case",
            "dead_code",
            "clippy::type_complexity",
            "renamed_and_removed_lints"
        ];

        for lint_ref in ALLOWED_LINTS {
            writeln!(&mut file, "#![allow({lint_ref})]").expect("unable to write the lint");
        }
    }

    fn link(&self) {
        println!(
            "cargo:rustc-link-search=native={}",
            self.build_dir.display()
        );
        println!("cargo:rustc-link-lib=static=sqlite3");
    }
}

fn main() {
    let sqlite3_builder = SQLiteBuilder::new();
    sqlite3_builder.build();
    sqlite3_builder.link();
    sqlite3_builder.bindgen();
}
