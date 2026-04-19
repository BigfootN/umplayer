use std::process::Command;

const BUILD_DIR: &str = "sqlite3_build";

const SQLITE_DIR: &str = "../sqlite";
const CONFIGURE_PATH: &str = "../../sqlite/configure";

fn main() {
    println!("cargo:rerun-if-changed={}", SQLITE_DIR);
    println!("cargo:rerun-if-changed={}", "build.rs");
    println!("cargo:rerun-if-changed={}", "libsqlite3.a");

    build_sqlite_static_lib();
    link_libsqlite3();
}

fn create_sqlite_build_dir() {
    std::fs::create_dir(BUILD_DIR).expect("unable to create sqlite build dir");
}

fn configure_sqlite_build() {
    create_sqlite_build_dir();
    Command::new(CONFIGURE_PATH).current_dir(BUILD_DIR).output().expect("unable to configure sqlite");
}

fn build_sqlite_static_lib() {
    configure_sqlite_build();
    Command::new("make").arg("libsqlite3.a").current_dir(BUILD_DIR).output().expect("unable to build libsqlite3.a");
}

fn link_libsqlite3() {
    println!("cargo:rustc-link-search=native={}", BUILD_DIR);
    println!("cargo:rustc-link-lib=static=sqlite3");
}
