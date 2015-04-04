extern crate pkg_config;

use std::env;
use std::fs;
use std::path::{Path};
use std::process::{Command};
use std::io::{self,Write};

static ARCHIVE: &'static str = "libsass.a";
static PROJECT: &'static str = "libsass";

fn main() {
    match pkg_config::find_library("sass") {
        Ok(_) => return,
        Err(_) => {}
    }
    let mut stderr = io::stderr();
    let src = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join(PROJECT);
    let archive = src.join("lib").join(ARCHIVE);
    // writeln!(&mut stderr,"looking for {}", archive.display()).unwrap();
    if !fs::metadata(archive.as_path()).is_ok() {
        let mut make = Command::new("make");
        make.current_dir(&src);
        // writeln!(&mut stderr,"running: {:?}", make).unwrap();
        let _ = make.status().unwrap();
    }
    // writeln!(&mut stderr, "validating that archive exists").unwrap();
    assert!(fs::metadata(archive.as_path()).is_ok(), "Error: archive does not exist after build");

    // copy to the output folder
    let out = &env::var("OUT_DIR").unwrap();
    let dst = Path::new(out);
    // writeln!(&mut stderr, "creating {}",dst.display()).unwrap();
    let _ = fs::create_dir_all(&dst).unwrap();
    match fs::copy(&archive, &dst.join(ARCHIVE)) {
        Ok(_) => {},
        Err(a) => {
            writeln!(&mut stderr,
                        "Error {:?} when copying \n{} \nto {}", a,
                        archive.display(), dst.display()).unwrap();
            panic!("copy failed");
            }
    }


    println!("cargo:rustc-flags=-L native={} -l static=sass -l dylib=stdc++",dst.display());

}
