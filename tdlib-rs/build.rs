// Copyright 2020 - developers of the `grammers` project.
// Copyright 2021 - developers of the `tdlib-rs` project.
// Copyright 2024 - developers of the `tgt` and `tdlib-rs` projects.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#[cfg(not(any(feature = "docs", feature = "pkg-config")))]
use lazy_static::lazy_static;
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
const TDLIB_VERSION: &str = "1.8.19";

#[cfg(not(any(feature = "docs", feature = "pkg-config")))]
/// The build configuration.
struct BuildConfig {
    /// It can be:
    ///   - the downloaded tdlib path, automatically downloaded from the github release
    ///   - the system tdlib path, setted by the user using the LOCAL_TDLIB_PATH env variable
    // tdlib_path: Option<String>,
    // /// The prefix where the tdlib will be copied. It is the concatenation of the out_dir and the `tdlib` folder name.
    // prefix: String,
    /// The include directory where the tdlib headers are placed.
    /// It is the concatenation of the prefix and the `include` folder name.
    include_dir: String,
    /// The lib directory where the tdlib shared libraries are placed.
    /// It is the concatenation of the prefix and the `lib` folder name.
    lib_dir: String,
    /// The bin directory where the tdlib binaries are placed.
    /// It is the concatenation of the prefix and the `bin` folder name.
    bin_dir: Option<String>,
    /// The shared library file path.
    lib_path: String,
}

#[cfg(not(any(feature = "docs", feature = "pkg-config", not(feature = "local-tdlib"))))]
fn get_tdlib_path() -> Option<String> {
    if cfg!(feature = "local-tdlib") {
        match env::var("LOCAL_TDLIB_PATH") {
            Ok(path) => Some(path),
            Err(_) => {
                panic!(
                    "The LOCAL_TDLIB_PATH env variable must be set to the path of the tdlib folder"
                );
            }
        }
    } else {
        // download_tdlib();
        None
    }
}

#[cfg(not(any(feature = "docs", feature = "pkg-config")))]
lazy_static! {
    static ref BUILD_CONFIG: BuildConfig = {
        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        {
            let out_dir = env::var("OUT_DIR").unwrap();
            // let tdlib_path = get_tdlib_path(); // "/home/fcb/lib/tdlib".to_string()
            let prefix = format!("{}/tdlib", out_dir);
            let include_dir = format!("{}/include", prefix);
            let lib_dir = format!("{}/lib", prefix);
            let bin_dir = None;
            let lib_path = format!("{}/libtdjson.so.{}", lib_dir, TDLIB_VERSION);

            BuildConfig {
                // tdlib_path,
                // prefix,
                include_dir,
                lib_dir,
                bin_dir,
                lib_path,
            }
        }

        #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
        {
            let out_dir = env::var("OUT_DIR").unwrap();
            // let tdlib_path = get_tdlib_path();// r"C:\Users\andre\Documents\tdlib\td\tdlib".to_string()
            let prefix = format!(r"{}\tdlib", out_dir);
            let include_dir = format!(r"{}\include", prefix);
            let lib_dir = format!(r"{}\lib", prefix);
            let bin_dir = Some(format!(r"{}\bin", prefix));
            let lib_path = format!(r"{}\tdjson.lib", lib_dir);

            BuildConfig {
                // tdlib_path,
                // prefix,
                include_dir,
                lib_dir,
                bin_dir,
                lib_path,
            }
        }

        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        {
            let out_dir = env::var("OUT_DIR").unwrap();
            // let tdlib_path = get_tdlib_path();// "/Users/federicobruzzone/lib/tdlib".to_string()
            let prefix = format!("{}/tdlib", out_dir);
            let include_dir = format!("{}/include", prefix);
            let lib_dir = format!("{}/lib", prefix);
            let bin_dir = None;
            let lib_path = format!("{}/libtdjson.{}.dylib", lib_dir, TDLIB_VERSION);

            BuildConfig {
                // tdlib_path,
                // prefix,
                include_dir,
                lib_dir,
                bin_dir,
                lib_path,
            }
        }

        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        {
            let out_dir = env::var("OUT_DIR").unwrap();
            // let tdlib_path = get_tdlib_path(); //"/Users/federicobruzzone/lib/tdlib";
            let prefix = format!("{}/tdlib", out_dir);
            let include_dir = format!("{}/include", prefix);
            let lib_dir = format!("{}/lib", prefix);
            let bin_dir = None;
            let lib_path = format!("{}/libtdjson.{}.dylib", lib_dir, TDLIB_VERSION);

            BuildConfig {
                // tdlib_path,
                // prefix,
                include_dir,
                lib_dir,
                bin_dir,
                lib_path,
            }
        }
    };
}

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

#[cfg(not(any(feature = "docs", feature = "pkg-config")))]
/// Build the project using the generic build configuration.
/// The current supported platforms are:
/// - Linux x86_64
/// - Windows x86_64
/// - MacOS x86_64
/// - MacOS aarch64
fn generic_build() {
    let build_config = &*BUILD_CONFIG;

    #[cfg(feature = "local-tdlib")]
    {
        let tdlib_path = get_tdlib_path();
        if let Some(tdlib_path) = tdlib_path {
            /// The prefix where the tdlib will be copied. It is the concatenation of the out_dir and the `tdlib` folder name.
            let prefix = format!("{}/tdlib", env::var("OUT_DIR").unwrap());
            let _ = copy_dir_all(Path::new(&tdlib_path), Path::new(&prefix));
        }
    }
    #[cfg(not(feature = "local-tdlib"))]
    download_tdlib();

    let lib_path = &build_config.lib_path;
    if !std::path::PathBuf::from(lib_path.clone()).exists() {
        panic!("tdjson shared library not found at {}", lib_path);
    }

    let bin_dir = &build_config.bin_dir;
    if let Some(bin_dir) = bin_dir {
        println!("cargo:rustc-link-search=native={}", bin_dir);
    }

    let lib_dir = &build_config.lib_dir;
    println!("cargo:rustc-link-search=native={}", lib_dir);

    let include_dir = &build_config.include_dir;
    println!("cargo:include={}", include_dir);

    println!("cargo:rustc-link-lib=dylib=tdjson");
}

#[cfg(not(any(feature = "docs", feature = "pkg-config", feature = "local-tdlib")))]
fn download_tdlib() {
    let base_url = "https://github.com/FedericoBruzzone/tdlib-rs/releases/download";
    // let url = format!(
    //     "{}/v{}/TDLib-{}-{}-{}.zip",
    //     base_url,
    //     env!("CARGO_PKG_VERSION"),
    //     TDLIB_VERSION,
    //     std::env::var("CARGO_CFG_TARGET_OS").unwrap(),
    //     std::env::var("CARGO_CFG_TARGET_ARCH").unwrap(),
    // );
    let url = format!(
        "{}/test/{}-{}-TDLib-{}.zip",
        base_url,
        "Windows", //"Linux" //"macOS"
        "X64",     // "ARM64"
        "2589c3fd46925f5d57e4ec79233cd1bd0f5d0c09"
    );

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
    #[cfg(all(feature = "docs", feature = "local-tdlib"))]
    compile_error!(
        "feature \"docs\" and feature \"local-tdlib\" cannot be enabled at the same time"
    );
    #[cfg(all(feature = "pkg-config", feature = "local-tdlib"))]
    compile_error!(
        "feature \"pkg-config\" and feature \"local-tdlib\" cannot be enabled at the same time"
    );

    #[cfg(feature = "local-tdlib")]
    println!("cargo:rerun-if-env-changed=LOCAL_TDLIB_PATH");

    println!("cargo:rerun-if-changed=build.rs");

    // Prevent linking libraries to avoid documentation failure
    #[cfg(not(feature = "docs"))]
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
            lazy_static::initialize(&BUILD_CONFIG);
            generic_build();
        }
    }

    let out_dir = env::var("OUT_DIR").unwrap();

    let definitions = load_tl("tl/api.tl")?;

    let mut file = BufWriter::new(File::create(Path::new(&out_dir).join("generated.rs"))?);

    generate_rust_code(&mut file, &definitions, cfg!(feature = "bots-only-api"))?;

    file.flush()?;

    Ok(())
}
