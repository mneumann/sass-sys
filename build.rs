extern crate "pkg-config" as pkg_config;
use std::os;
use std::old_io::{self, fs, Command};
use std::old_io::process::InheritFd;

fn main() {
    match pkg_config::find_library("sass") {
        Ok(_) => return,
        Err(_) => {}
    }
    let src = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap());
    let mut cmd = Command::new("make");
    cmd.cwd(&src.join("libsass"));
    run(& mut cmd);

    println!("cargo:rustc-flags=-l sass -L libsass/lib/ -l dylib=stdc++");

}

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    assert!(cmd.stdout(InheritFd(1))
               .stderr(InheritFd(2))
               .status()
               .unwrap()
               .success());

}
