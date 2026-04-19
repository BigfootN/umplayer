use std::process::Command;

struct SQLiteBuilder {
    build_dir: std::path::PathBuf,
    configure_path: std::path::PathBuf,
}

impl SQLiteBuilder {
    const BUILD_DIR: &'static str = "sqlite3_build";

    const SOURCE_DIR: &'static str = "sqlite_c_src";

    fn new() -> Self {
        let current_dir = std::env::current_dir().expect("unable to get current directory");
        let build_dir = current_dir.join(Self::BUILD_DIR);
        let script_path = current_dir.join("build.rs");
        let configure_path = current_dir.join(Self::SOURCE_DIR).join("configure");
        let sqlite_3_h_path = current_dir.join(Self::SOURCE_DIR).join("sqlite3.h");
        let sqlite_3_c_path = current_dir.join(Self::SOURCE_DIR).join("sqlite3.c");
        let makefile_path = current_dir.join(Self::SOURCE_DIR).join("makefile");

        println!("cargo:rerun-if-changed={}", script_path.display());
        println!("cargo:rerun-if-changed={}", configure_path.display());
        println!("cargo:rerun-if-changed={}", sqlite_3_h_path.display());
        println!("cargo:rerun-if-changed={}", sqlite_3_c_path.display());
        println!("cargo:rerun-if-changed={}", makefile_path.display());

        Self {
            build_dir,
            configure_path,
        }
    }

    fn build(&self) {
        self.create_build_dir();
        self.configure_build();
        Command::new(&self.configure_path).current_dir(&self.build_dir).output().expect("failed to run configure");
    }

    fn create_build_dir(&self) {
        if !self.build_dir.exists() {
            std::fs::create_dir(&self.build_dir).expect("failed to create build dir");
        }
    }

    fn configure_build(&self) {
        Command::new(&self.configure_path).current_dir(&self.build_dir).output().expect("failed to run configure");
    }

    fn link(&self) {
        println!("cargo:rustc-link-search=native={}", self.build_dir.display());
        println!("cargo:rustc-link-lib=static=sqlite3");
    }
}

fn main() {
    let sqlite3_builder = SQLiteBuilder::new();
    sqlite3_builder.build();
    sqlite3_builder.link();
}
