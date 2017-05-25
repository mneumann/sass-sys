extern crate pkg_config;

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::io::{self, Write};

static PROJECT: &'static str = "libsass";

static ARCHIVE: &'static str = "libsass.a";
static ARCHIVE_WINDOWS: &'static str = "libsass.lib";

fn main() {
    // See if sass is already setup
    match pkg_config::find_library("sass") {
        Ok(_) => return,
        Err(_) => {}
    }

    // Setup some paths
    let target = env::var("TARGET").expect("TARGET not found");
    let manifest = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not found");
    let src = Path::new(&manifest).join(PROJECT);
    let is_windows = target.contains("windows");
    let is_darwin = target.contains("darwin");
    let archive = if is_windows {
        src.join("win").join("bin").join(ARCHIVE_WINDOWS)
    } else {
        src.join("lib").join(ARCHIVE)
    };

    // Run make on libsass
    if !fs::metadata(archive.as_path()).is_ok() {
        if !is_windows {
            let mut make = Command::new("make");
            make.current_dir(&src);
            let _ = make.status().expect("Couldn't get status of make");
        } else {
            let mut msbuild = Command::new("msbuild");
            msbuild.arg("win\\libsass.sln");
            msbuild.arg("/p:LIBSASS_STATIC_LIB=1");
            msbuild.arg("/p:Configuration=Release");
            msbuild.current_dir(&src);
            let _ = msbuild.status().expect("Couldn't get status of msbuild");
        }
    }

    // Verify that libsass was build correctly
    assert!(fs::metadata(archive.as_path()).is_ok(),
            "Error: archive does not exist after build");

    // Setup output directory
    let out = &env::var("OUT_DIR").expect("OUT_DIR not found");
    let dst = Path::new(out);
    let _ = fs::create_dir_all(&dst).expect("Cannot create destination directory");

    // Copy archive to output directory
    match fs::copy(&archive, &dst.join(if is_windows { ARCHIVE_WINDOWS } else { ARCHIVE })) {
        Ok(_) => {}
        Err(a) => {
            let mut stderr = io::stderr();
            writeln!(&mut stderr,
                     "Error {:?} when copying {} to {}",
                     a,
                     archive.display(),
                     dst.display())
                .unwrap();
            panic!("copy failed");
        }
    }

    // Link to libsass
    println!(
        "cargo:rustc-flags=-L native={} -l static={} -l dylib={}",
        dst.display(),
        if is_windows { "libsass" } else { "sass" },
        if is_darwin { "c++" } else { "stdc++ "}
    );
}
