// From: https://docs.rs/crate/gdcm_conv/0.1.0/s
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn build() {
    // run GDCM cmake
    let mut cfg = cmake::Config::new("GDCM");

    let dst = cfg
        .define("GDCM_BUILD_TESTING", "OFF")
        .define("GDCM_DOCUMENTATION", "OFF")
        .define("GDCM_BUILD_EXAMPLES", "OFF")
        .define("GDCM_BUILD_SHARED_LIBS", "OFF")
        .define("GDCM_BUILD_DOCBOOK_MANPAGES", "OFF")
        .define("CMAKE_CXX_STANDARD", "14")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("CMAKE_CXX_FLAGS", "-O3 -march=native -flto -fvisibility=default")
        .define("CMAKE_C_FLAGS", "-O3 -march=native -flto")
        .define("CMAKE_EXE_LINKER_FLAGS", "-flto")
        .build();

    // set GDCM include path
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let include_dir = out_path.join("include").join("gdcm-3.0");

    // create library
    cc::Build::new()
        .file("gdcm_wrapper.cc")
        .cpp(true)
        .std("c++14")
        .include(include_dir)
        .compile("gdcm_wrapper");

    println!("cargo:rustc-link-arg=-Wl,-Bstatic");
    
    // set libs paths
    //println!("cargo:rustc-link-search={}", dst.join("lib").display());
    //println!("cargo:rustc-link-search={}", dst.display());
    println!("cargo:rustc-link-search=native=/usr/local/lib/");
    
    // set libs
    println!("cargo:rustc-link-lib=static=gdcm_wrapper");

    // gdcm libs
    println!("cargo:rustc-link-lib=static=gdcmMSFF");
    println!("cargo:rustc-link-lib=static=gdcmCommon");
    println!("cargo:rustc-link-lib=static=gdcmDICT");
    println!("cargo:rustc-link-lib=static=gdcmDSED");
    println!("cargo:rustc-link-lib=static=gdcmIOD");
    println!("cargo:rustc-link-lib=static=gdcmexpat");
    println!("cargo:rustc-link-lib=static=gdcmjpeg12");
    println!("cargo:rustc-link-lib=static=gdcmjpeg16");
    println!("cargo:rustc-link-lib=static=gdcmjpeg8");
    println!("cargo:rustc-link-lib=static=gdcmopenjp2");
    
    if env::consts::OS != "windows" {
        println!("cargo:rustc-link-lib=static=gdcmuuid");
    }
    println!("cargo:rustc-link-lib=static=gdcmMEXD");
    println!("cargo:rustc-link-lib=static=gdcmzlib");

    #[cfg(feature = "charls")]
    println!("cargo:rustc-link-lib=static=gdcmcharls");

    println!("Building for {}", env::consts::OS);
    match env::consts::OS {
        "macos" => {
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-search=framework=/System/Library/Frameworks");
        }
        "windows" => {
            println!("cargo:rustc-link-lib=dylib=rpcrt4");
            println!("cargo:rustc-link-lib=dylib=crypt32");
            println!("cargo:rustc-link-lib=static=socketxx");
        }
        _ => {
            println!("cargo:rustc-link-lib=dylib=stdc++");
        }
    };
}

fn main() {
    // re-build if files change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=gdcm_wrapper.cc");
    println!("cargo:rerun-if-changed=wrapper.h");

    // unset DESTDIR envar to avoid others libs destinations
    env::remove_var("DESTDIR");

    // update git
    if !Path::new("GDCM/.git").exists() {
        let _ = Command::new("git")
            .args(["submodule", "update", "--init"])
            .status();
    }

    build();
}
