extern crate pkg_config;

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::io::{self,Write};

static ARCHIVE: &'static str = "libsass.a";
static PROJECT: &'static str = "libsass";

fn main() {
    // See if sass is already setup
    match pkg_config::find_library("sass") {
        Ok(_) => return,
        Err(_) => {}
    }

    // Setup some paths
    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    let src = Path::new(&manifest).join(PROJECT);
    let archive = src.join("lib").join(ARCHIVE);

    // Run make on libsass
    if !fs::metadata(archive.as_path()).is_ok() {
        let mut make = Command::new("make");
        make.current_dir(&src);
        let _ = make.status().unwrap();
    }

    // Verify that libsass was build correctly
    assert!(fs::metadata(archive.as_path()).is_ok(),
            "Error: archive does not exist after build");

    // Setup output directory
    let out = &env::var("OUT_DIR").unwrap();
    let dst = Path::new(out);
    let _ = fs::create_dir_all(&dst).unwrap();

    // Copy archive to output directory
    match fs::copy(&archive, &dst.join(ARCHIVE)) {
        Ok(_) => {},
        Err(a) => {
            let mut stderr = io::stderr();
            writeln!(&mut stderr,
                     "Error {:?} when copying {} to {}",
                     a, archive.display(), dst.display()).unwrap();
            panic!("copy failed");
        }
    }

    // Link to libsass
    println!("cargo:rustc-flags=-L native={} -l static=sass -l dylib=stdc++",
             dst.display());
}
