// Copyright 2020 - developers of the `grammers` project.
// Copyright 2021 - developers of the `tdlib-rs` project.
// Copyright 2024 - developers of the `tgt` and `tdlib-rs` projects.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use std::env;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::Path;
use tdlib_rs_gen::generate_rust_code;
use tdlib_rs_parser::parse_tl_file;
use tdlib_rs_parser::tl::Definition;

#[allow(dead_code)]
#[cfg(not(any(feature = "docs", feature = "pkg-config")))]
/// The version of the TDLib library.
const TDLIB_VERSION: &str = "1.8.29";

/// Load the type language definitions from a certain file.
/// Parse errors will be printed to `stderr`, and only the
/// valid results will be returned.
fn load_tl(file: &str) -> std::io::Result<Vec<Definition>> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(parse_tl_file(contents)
        .filter_map(|d| match d {
            Ok(d) => Some(d),
            Err(e) => {
                eprintln!("TL: parse error: {:?}", e);
                None
            }
        })
        .collect())
}

#[cfg(feature = "local-tdlib")]
/// Copy all files from a directory to another.
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

#[cfg(feature = "local-tdlib")]
/// Copy all the tdlib folder find in the LOCAL_TDLIB_PATH environment variable to the OUT_DIR/tdlib folder
fn copy_local_tdlib() {
    match env::var("LOCAL_TDLIB_PATH") {
        Ok(tdlib_path) => {
            let out_dir = env::var("OUT_DIR").unwrap();
            let prefix = format!("{}/tdlib", out_dir);
            copy_dir_all(Path::new(&tdlib_path), Path::new(&prefix)).unwrap();
        }
        Err(_) => {
            panic!("The LOCAL_TDLIB_PATH env variable must be set to the path of the tdlib folder");
        }
    };
}

#[cfg(any(feature = "download-tdlib", feature = "local-tdlib"))]
/// Build the project using the generic build configuration.
/// The current supported platforms are:
/// - Linux x86_64
/// - Windows x86_64
/// - MacOS x86_64
/// - MacOS aarch64
fn generic_build() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let prefix = format!("{}/tdlib", out_dir);
    let include_dir = format!("{}/include", prefix);
    let lib_dir = format!("{}/lib", prefix);
    let lib_path = {
        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        {
            format!("{}/libtdjson.so.{}", lib_dir, TDLIB_VERSION)
        }
        #[cfg(any(
            all(target_os = "macos", target_arch = "x86_64"),
            all(target_os = "macos", target_arch = "aarch64")
        ))]
        {
            format!("{}/libtdjson.{}.dylib", lib_dir, TDLIB_VERSION)
        }
        #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
        {
            format!(r"{}\tdjson.lib", lib_dir)
        }
    };

    if !std::path::PathBuf::from(lib_path.clone()).exists() {
        panic!("tdjson shared library not found at {}", lib_path);
    }

    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        let bin_dir = format!(r"{}\bin", prefix);
        println!("cargo:rustc-link-search=native={}", bin_dir);
    }

    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:include={}", include_dir);
    println!("cargo:rustc-link-lib=dylib=tdjson");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir);
}

#[cfg(feature = "download-tdlib")]
fn download_tdlib() {
    let base_url = "https://github.com/FedericoBruzzone/tdlib-rs/releases/download";
    let url = format!(
        "{}/v{}/tdlib-{}-{}-{}.zip",
        base_url,
        env!("CARGO_PKG_VERSION"),
        TDLIB_VERSION,
        std::env::var("CARGO_CFG_TARGET_OS").unwrap(),
        std::env::var("CARGO_CFG_TARGET_ARCH").unwrap(),
    );
    // let target_os = if cfg!(target_os = "windows") {
    //     "Windows"
    // } else if cfg!(target_os = "linux") {
    //     "Linux"
    // } else if cfg!(target_os = "macos") {
    //     "macOS"
    // } else {
    //     ""
    // };
    // let target_arch = if cfg!(target_arch = "x86_64") {
    //     "X64"
    // } else if cfg!(target_arch = "aarch64") {
    //     "ARM64"
    // } else {
    //     ""
    // };
    // let url = format!(
    //     "{}/test/{}-{}-TDLib-{}.zip",
    //     base_url, target_os, target_arch, "2589c3fd46925f5d57e4ec79233cd1bd0f5d0c09"
    // );

    let out_dir = env::var("OUT_DIR").unwrap();
    let tdlib_dir = format!("{}/tdlib", &out_dir);
    let zip_path = format!("{}.zip", &tdlib_dir);

    // Create the request
    let response = reqwest::blocking::get(&url).unwrap();

    // Check if the response status is successful
    if response.status().is_success() {
        // Create a file to write to
        let mut dest = File::create(&zip_path).unwrap();

        // Get the response bytes and write to the file
        let content = response.bytes().unwrap();
        std::io::copy(&mut content.as_ref(), &mut dest).unwrap();
    } else {
        panic!(
            "[{}] Failed to download file: {}\n{}\n{}",
            "Your OS or architecture may be unsupported.",
            "Please try using the `pkg-config` or `local-tdlib` features.",
            response.status(),
            &url
        )
    }

    let mut archive = zip::ZipArchive::new(File::open(&zip_path).unwrap()).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = Path::new(&out_dir).join(file.name());

        if (*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    let _ = std::fs::remove_file(&zip_path);
}

fn main() -> std::io::Result<()> {
    #[cfg(all(feature = "docs", feature = "pkg-config"))]
    compile_error!(
        "feature \"docs\" and feature \"pkg-config\" cannot be enabled at the same time"
    );
    #[cfg(all(feature = "docs", feature = "download-tdlib"))]
    compile_error!(
        "feature \"docs\" and feature \"download-tdlib\" cannot be enabled at the same time"
    );
    #[cfg(all(feature = "pkg-config", feature = "download-tdlib"))]
    compile_error!(
        "feature \"pkg-config\" and feature \"download-tdlib\" cannot be enabled at the same time"
    );

    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(feature = "local-tdlib")]
    println!("cargo:rerun-if-env-changed=LOCAL_TDLIB_PATH");

    // Prevent linking libraries to avoid documentation failure
    #[cfg(not(feature = "docs"))]
    {
        // It requires the following variables to be set:
        // - export PKG_CONFIG_PATH=$HOME/lib/tdlib/lib/pkgconfig/:$PKG_CONFIG_PATH
        // - export LD_LIBRARY_PATH=$HOME/lib/tdlib/lib/:$LD_LIBRARY_PATH
        #[cfg(feature = "pkg-config")]
        system_deps::Config::new().probe().unwrap();

        #[cfg(feature = "download-tdlib")]
        download_tdlib();

        // It requires the following variable to be set:
        // - export LOCAL_TDLIB_PATH=$HOME/lib/tdlib
        #[cfg(feature = "local-tdlib")]
        copy_local_tdlib();

        #[cfg(any(feature = "download-tdlib", feature = "local-tdlib"))]
        generic_build();
    }

    let out_dir = env::var("OUT_DIR").unwrap();

    let definitions = load_tl("tl/api.tl")?;

    let mut file = BufWriter::new(File::create(Path::new(&out_dir).join("generated.rs"))?);

    generate_rust_code(&mut file, &definitions, cfg!(feature = "bots-only-api"))?;

    file.flush()?;

    Ok(())
}
