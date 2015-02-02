extern crate "pkg-config" as pkg_config;

fn main() {
    match pkg_config::find_library("sass") {
        Ok(_) => return,
        Err(_) => {}
    }
    println!("cargo:rustc-flags=-l sass -L ../../sass/libsass/lib/");
}
