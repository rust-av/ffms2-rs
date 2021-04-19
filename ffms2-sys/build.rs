extern crate bindgen;
extern crate metadeps;

use env::VarError;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn format_write(builder: bindgen::Builder) -> String {
    builder
        .generate()
        .unwrap()
        .to_string()
        .replace("/**", "/*")
        .replace("/*!", "/*")
}

fn main() {
    // Consider 'FFMS_INCLUDE_DIR' and 'FFMS_LIB_DIR', if pkg-config should not be used.
    let headers = env::var("FFMS_INCLUDE_DIR").map(|value| {
        // Ensure the include directory is valid
        let include_dir = PathBuf::from(value.as_str());
        if !include_dir.is_dir() {
            panic!("The specified include directory '{}' in FFMS_INCLUDE_DIR is not valid.", value);
        }

        // Get the path to the lib.
        let lib_dir = env::var("FFMS_LIB_DIR").and_then(|lib_dir| {
            match PathBuf::from(lib_dir) {
                lib_dir if lib_dir.is_dir() => Ok(lib_dir),
                _ => Err(VarError::NotPresent)
            }
        }).expect("FFMS_LIB_DIR is not set or the specified directory is not valid.");

        // Using dynamic library in Windows remains a problem. We have to copy the DLL into a path...
        // Problem: If 'FFMS_LIB_DIR' is outside of 'target', it is not considered (https://doc.rust-lang.org/cargo/reference/environment-variables.html).
        #[cfg(windows)] {
            let cargo_output_dir = env::var("OUT_DIR").expect("Unable to get OUT_DIR");
            // We need to add the file in target/{debug|release as it is included in the PATH: https://doc.rust-lang.org/cargo/reference/environment-variables.html#dynamic-library-paths
            let linkable_dll: PathBuf = [cargo_output_dir.as_ref(), "..", "..", "..", "ffms2.dll"].iter().collect();

            // Copy the file if it does not exists
            if !linkable_dll.is_file() {
                let dll_file = lib_dir.as_path().join("ffms2.dll");
                if !dll_file.is_file() {
                    panic!("Unable to find the 'ffms2.dll' in 'FFMS_LIB_DIR' ('{}').", lib_dir.display());
                }

                std::fs::copy(dll_file, linkable_dll).expect("Copying DLL failed");     
            }
        }

        // Add the flags for cargo otherwise explicitely added by pkg-config-rs
        println!("cargo:rustc-link-lib=dylib=ffms2");
        println!("cargo:rustc-link-search=native={}", lib_dir.to_string_lossy());

        vec![include_dir]
    }).unwrap_or_else(|_| {
        let libs = metadeps::probe().expect("Unable to query include paths using pkg-config. Consider setting the environment variable FFMS_INCLUDE_DIR and FFMS_LIB_DIR explicitely.");
        libs.get("ffms2").unwrap().include_paths.clone()
    });

    let mut builder = bindgen::builder().header("data/ffms.h");
    for header in headers {
        builder = builder.clang_arg("-I").clang_arg(header.to_str().unwrap());
    }

    builder = builder.default_enum_style(bindgen::EnumVariation::Rust {
        non_exhaustive: false,
    });

    // Manually fix the comment so rustdoc won't try to pick them
    let s = format_write(builder);

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut file = File::create(out_path.join("ffms2.rs")).unwrap();

    let _ = file.write(s.as_bytes());
}
