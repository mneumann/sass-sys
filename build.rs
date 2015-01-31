extern crate "pkg-config" as pkg_config;

fn main() {
    match pkg_config::find_library("libsass1") {
        Ok(_) => return,
        Err(_) => {
            if cfg!(windows) {
                println!("cargo:rustc-flags=-l libsass:dylib");
            } else {
                println!("cargo:rustc-flags=-l libsaas:dylib");
            }
        }
    }
}
