extern crate bindgen;
extern crate pkg_config;

use std::fs;
use std::env;
use std::path;
use std::process;
use std::io::{ Write, };


fn main() {
    let current_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_path = path::PathBuf::from(env::var("OUT_DIR").unwrap());

    let fdkaac_source_dir = path::Path::new(&current_dir).join("fdk-aac");
    
    // bash ./autogen.sh
    // bash ./configure --enable-static=yes
    // make
    // ./libtool --mode=install install -c libfdk-aac.la ./lib/
    if fdkaac_source_dir.join("include").exists() == false {
        process::Command::new("mkdir")
            .arg("include")
            .current_dir(&fdkaac_source_dir)
            .status()
            .expect("failed to execute `mkdir include` process");
        process::Command::new("cp")
            .arg("libAACdec/include/aacdecoder_lib.h")
            .arg("include")
            .current_dir(&fdkaac_source_dir)
            .status()
            .expect("failed to execute `cp libAACdec/include/aacdecoder_lib.h include/` process");
        process::Command::new("cp")
            .arg("libAACenc/include/aacenc_lib.h")
            .arg("include")
            .current_dir(&fdkaac_source_dir)
            .status()
            .expect("failed to execute `cp libAACenc/include/aacenc_lib.h include` process");
        process::Command::new("cp")
            .arg("libSYS/include/FDK_audio.h")
            .arg("libSYS/include/audio.h")
            .arg("libSYS/include/cmdl_parser.h")
            .arg("libSYS/include/conv_string.h")
            .arg("libSYS/include/genericStds.h")
            .arg("libSYS/include/machine_type.h")
            .arg("libSYS/include/wav_file.h")
            .arg("include")
            .current_dir(&fdkaac_source_dir)
            .status()
            .expect("failed to execute `cp libSYS/include/* include` process");
    }

    if fdkaac_source_dir.join("lib").exists() == false {
        process::Command::new("mkdir")
            .arg("lib")
            .current_dir(&fdkaac_source_dir)
            .status()
            .expect("failed to execute `mkdir include` process");
    }

    if fdkaac_source_dir.join("Makefile").exists() == false {
        process::Command::new("bash")
            .arg("autogen.sh")
            .current_dir(&fdkaac_source_dir)
            .status()
            .expect("failed to execute `./autogen.sh` process");
        process::Command::new("bash")
            .arg("configure")
            .arg("--enable-static=yes")
            .current_dir(&fdkaac_source_dir)
            .status()
            .expect("failed to execute `./configure` process");
    }
    
    process::Command::new("make")
        .current_dir(&fdkaac_source_dir)
        .status()
        .expect("failed to execute `make` process");
    
    process::Command::new(&fdkaac_source_dir.join("libtool"))
        .arg("--mode=install")
        .arg("install")
        .arg("-c")
        .arg("libfdk-aac.la")
        .arg(format!("{}", &fdkaac_source_dir.join("lib").display()))
        .current_dir(&fdkaac_source_dir)
        .status()
        .expect("failed to execute `./libtool --mode=install install -c libfdk-aac.la lib/` process");

    assert_eq!(fdkaac_source_dir.join("lib").join("libfdk-aac.a").exists(), true);
    assert_eq!(fdkaac_source_dir.join("lib").join("libfdk-aac.la").exists(), true);


    let fdk_aac_header = format!("
#include <stdbool.h>
#include \"{}/include/aacdecoder_lib.h\"
#include \"{}/include/aacenc_lib.h\"
", &fdkaac_source_dir.clone().as_path().to_string_lossy(),
    &fdkaac_source_dir.clone().as_path().to_string_lossy());

    let mut file = fs::OpenOptions::new().write(true).create(true).append(false)
        .open(&out_path.join("fdkaac.h").as_path())
        .unwrap();
    file.write_all(&fdk_aac_header.as_bytes()).unwrap();

    bindgen::Builder::default()
        .header(out_path.join("fdkaac.h").as_path().to_string_lossy())
        // .impl_debug(true)
        // .impl_partialeq(true)
        .derive_copy(true)
        .derive_debug(true)
        .derive_default(true)
        .derive_hash(true)
        .derive_partialeq(true)
        .derive_eq(true)
        .layout_tests(false)
        .prepend_enum_name(false)
        // .default_enum_style(bindgen::EnumVariation::Rust)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("fdkaac.rs").as_path())
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-search=static={}", &fdkaac_source_dir.join("lib").display());
    println!("cargo:rustc-flags=-l dylib=stdc++");
    println!("cargo:rustc-flags=-l static=fdk-aac -L {}", &fdkaac_source_dir.join("lib").display());
    println!("cargo:rerun-if-changed={}", &fdkaac_source_dir.join("include").display());
    println!("cargo:rerun-if-changed={}", &fdkaac_source_dir.join("lib").display());
}
