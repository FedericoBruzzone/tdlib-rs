// Copyright 2021 - developers of the `tdlib-rs` project.
// Copyright 2020 - developers of the `grammers` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use std::env;
use std::fs::{self, File};
use std::io::{self, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
#[cfg(feature = "pkg-config")]
use system_deps;
use tdlib_tl_gen::generate_rust_code;
use tdlib_tl_parser::parse_tl_file;
use tdlib_tl_parser::tl::Definition;

/// Load the type language definitions from a certain file.
/// Parse errors will be printed to `stderr`, and only the
/// valid results will be returned.
fn load_tl(file: &str) -> io::Result<Vec<Definition>> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(parse_tl_file(contents)
        .into_iter()
        .filter_map(|d| match d {
            Ok(d) => Some(d),
            Err(e) => {
                eprintln!("TL: parse error: {:?}", e);
                None
            }
        })
        .collect())
}

#[cfg(not(feature = "pkg-config"))]
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn linux_x86_64() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let tdlib_download_path = "/home/fcb/lib/tdlib";

    let out_dir = Path::new(&out_dir);
    let prefix = format!("{}/tdlib", out_dir.to_str().unwrap());
    let _ = copy_dir_all(Path::new(&tdlib_download_path), Path::new(&prefix));

    let include_dir = format!("{}/include", prefix);
    let lib_dir = format!("{}/lib", prefix);
    let so_path = format!("{}/libtdjson.so.1.8.19", lib_dir);
    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-lib=dylib=tdjson");
    println!("cargo:include={}", include_dir);
    if !PathBuf::from(so_path.clone()).exists() {
        panic!("tdjson shared library not found at {}", so_path);
    }
}

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
fn windows_x86_64() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let tdlib_download_path = r"C:\Users\andre\Documents\tdlib\td\tdlib";

    let out_dir = Path::new(&out_dir);
    let prefix = format!("{}/tdlib", out_dir.to_str().unwrap());

    let _ = copy_dir_all(Path::new(&tdlib_download_path), Path::new(&prefix));

    println!("cargo:rustc-link-lib=dylib=tdjson");

    let lib_dir = format!("{}/lib", prefix);
    println!("cargo:rustc-link-search=native={}", lib_dir);

    // for the .dll
    let bin_dir = format!("{}/bin", prefix);
    println!("cargo:rustc-link-search=native={}", bin_dir);

    let include_dir = format!("{}/include", prefix);
    println!("cargo:include={}", include_dir);

    let lib_path = format!("{}/tdjson.lib", lib_dir);
    if !PathBuf::from(lib_path.clone()).exists() {
        panic!("tdjson shared library not found at {}", lib_path);
    }
}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
fn macos_x86_64() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let tdlib_download_path = "/Users/federicobruzzone/lib/tdlib";

    let out_dir = Path::new(&out_dir);
    let prefix = format!("{}/tdlib", out_dir.to_str().unwrap());
    let _ = copy_dir_all(Path::new(&tdlib_download_path), Path::new(&prefix));

    let include_dir = format!("{}/include", prefix);
    let lib_dir = format!("{}/lib", prefix);
    let so_path = format!("{}/libtdjson.1.8.19.dylib", lib_dir);
    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-lib=dylib=tdjson");
    println!("cargo:include={}", include_dir);
    if !PathBuf::from(so_path.clone()).exists() {
        panic!("tdjson shared library not found at {}", so_path);
    }
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn macos_aarch64() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let tdlib_download_path = "/Users/federicobruzzone/lib/tdlib";

    let out_dir = Path::new(&out_dir);
    let prefix = format!("{}/tdlib", out_dir.to_str().unwrap());
    let _ = copy_dir_all(Path::new(&tdlib_download_path), Path::new(&prefix));

    let include_dir = format!("{}/include", prefix);
    let lib_dir = format!("{}/lib", prefix);
    let so_path = format!("{}/libtdjson.1.8.19.dylib", lib_dir);
    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-lib=dylib=tdjson");
    println!("cargo:include={}", include_dir);
    if !PathBuf::from(so_path.clone()).exists() {
        panic!("tdjson shared library not found at {}", so_path);
    }
}

fn main() -> std::io::Result<()> {
    // TODO Get artifacts from github (lib folder and include folder)

    // Prevent linking libraries to avoid documentation failure
    #[cfg(not(feature = "dox"))]
    {
        // It requires the following variables to be set:
        // - export PKG_CONFIG_PATH=$HOME/lib/tdlib/lib/pkgconfig/:$PKG_CONFIG_PATH
        // - export LD_LIBRARY_PATH=$HOME/lib/tdlib/lib/:$LD_LIBRARY_PATH
        #[cfg(feature = "pkg-config")]
        {
            system_deps::Config::new().probe().unwrap();
        }

        #[cfg(not(feature = "pkg-config"))]
        {
            #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
            linux_x86_64();

            #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
            windows_x86_64();

            #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
            macos_x86_64();

            #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
            macos_aarch64();
        }
    }

    let out_dir = env::var("OUT_DIR").unwrap();

    let definitions = load_tl("tl/api.tl")?;

    let mut file = BufWriter::new(File::create(Path::new(&out_dir).join("generated.rs"))?);

    generate_rust_code(&mut file, &definitions, cfg!(feature = "bots-only-api"))?;

    file.flush()?;
    Ok(())
}
