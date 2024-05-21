// Copyright 2020 - developers of the `grammers` project.
// Copyright 2021 - developers of the `tdlib-rs` project.
// Copyright 2024 - developers of the `tgt` and `tdlib-rs` projects.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use lazy_static::lazy_static;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use tdlib_rs_gen::generate_rust_code;
use tdlib_rs_parser::parse_tl_file;
use tdlib_rs_parser::tl::Definition;

/// The version of the TDLib library.
#[allow(dead_code)]
const TDLIB_VERSION: &str = "1.8.19";

/// The build configuration.
struct BuildConfig {
    /// It can be:
    ///   - the downloaded tdlib path, automatically downloaded from the github release
    ///   - the system tdlib path, setted by the user using the LOCAL_TDLIB_PATH env variable
    tdlib_path: String,
    /// The prefix where the tdlib will be copied. It is the concatenation of the out_dir and the `tdlib` folder name.
    prefix: String,
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

lazy_static! {
    static ref BUILD_CONFIG: BuildConfig = {
        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        {
            let out_dir = env::var("OUT_DIR").unwrap();
            let tdlib_path = {
                if cfg!(feature = "local-tdlib") {
                    match env::var("LOCAL_TDLIB_PATH") {
                        Ok(path) => path,
                        Err(_) => {
                            panic!("The LOCAL_TDLIB_PATH env variable must be set to the path of the tdlib folder");
                        }
                    }
                } else {
                    "/home/fcb/lib/tdlib".to_string()
                }
            };
            let prefix = format!("{}/tdlib", out_dir);
            let include_dir = format!("{}/include", prefix);
            let lib_dir = format!("{}/lib", prefix);
            let bin_dir = None;
            let lib_path = format!("{}/libtdjson.so.{}", lib_dir, TDLIB_VERSION);

            BuildConfig {
                tdlib_path,
                prefix,
                include_dir,
                lib_dir,
                bin_dir,
                lib_path,
            }
        }

        #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
        {
            let out_dir = env::var("OUT_DIR").unwrap();
            let tdlib_path = {
                if cfg!(feature = "local-tdlib") {
                    match env::var("LOCAL_TDLIB_PATH") {
                        Ok(path) => path,
                        Err(_) => {
                            panic!("The LOCAL_TDLIB_PATH env variable must be set to the path of the tdlib folder");
                        }
                    }
                } else {
                    r"C:\Users\andre\Documents\tdlib\td\tdlib".to_string()
                }
            };
            let prefix = format!(r"{}\tdlib", out_dir);
            let include_dir = format!(r"{}\include", prefix);
            let lib_dir = format!(r"{}\lib", prefix);
            let bin_dir = Some(format!(r"{}\bin", prefix));
            let lib_path = format!(r"{}\tdjson.lib", lib_dir);

            BuildConfig {
                tdlib_path,
                prefix,
                include_dir,
                lib_dir,
                bin_dir,
                lib_path,
            }
        }

        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        {
            let out_dir = env::var("OUT_DIR").unwrap();
            let tdlib_path = {
                if cfg!(feature = "local-tdlib") {
                    match env::var("LOCAL_TDLIB_PATH") {
                        Ok(path) => path,
                        Err(_) => {
                            panic!("The LOCAL_TDLIB_PATH env variable must be set to the path of the tdlib folder");
                        }
                    }
                } else {
                    "/Users/federicobruzzone/lib/tdlib".to_string()
                }
            };
            let prefix = format!("{}/tdlib", out_dir);
            let include_dir = format!("{}/include", prefix);
            let lib_dir = format!("{}/lib", prefix);
            let bin_dir = None;
            let lib_path = format!("{}/libtdjson.{}.dylib", lib_dir, TDLIB_VERSION);

            BuildConfig {
                tdlib_path,
                prefix,
                include_dir,
                lib_dir,
                bin_dir,
                lib_path,
            }
        }

        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        {
            let out_dir = env::var("OUT_DIR").unwrap();
            let tdlib_download_path = {
                if cfg!(feature = "local-tdlib") {
                    match env::var("LOCAL_TDLIB_PATH") {
                        Ok(path) => path,
                        Err(_) => {
                            panic!("The LOCAL_TDLIB_PATH env variable must be set to the path of the tdlib folder");
                        }
                    }
                } else {
                    "/Users/federicobruzzone/lib/tdlib";
                };
            };
            let prefix = format!("{}/tdlib", out_dir);
            let include_dir = format!("{}/include", prefix);
            let lib_dir = format!("{}/lib", prefix);
            let bin_dir = None;
            let lib_path = format!("{}/libtdjson.{}.dylib", lib_dir, TDLIB_VERSION);

            BuildConfig {
                tdlib_path,
                prefix,
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
fn load_tl(file: &str) -> io::Result<Vec<Definition>> {
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

#[cfg(not(feature = "pkg-config"))]
/// Copy all files from a directory to another.
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

#[cfg(not(feature = "pkg-config"))]
/// Build the project using the generic build configuration.
/// The current supported platforms are:
/// - Linux x86_64
/// - Windows x86_64
/// - MacOS x86_64
/// - MacOS aarch64
fn generic_build() {
    let build_config = &*BUILD_CONFIG;

    let tdlib_path = &build_config.tdlib_path;
    let prefix = &build_config.prefix;
    let _ = copy_dir_all(Path::new(&tdlib_path), Path::new(&prefix));

    let lib_path = &build_config.lib_path;
    if !PathBuf::from(lib_path.clone()).exists() {
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
            lazy_static::initialize(&BUILD_CONFIG);
            generic_build();
        }
    }

    let out_dir = env::var("OUT_DIR").unwrap();

    let definitions = load_tl("tl/api.tl")?;

    let mut file = BufWriter::new(File::create(Path::new(&out_dir).join("generated.rs"))?);

    generate_rust_code(&mut file, &definitions, cfg!(feature = "bots-only-api"))?;

    file.flush()?;

    #[cfg(feature = "local-tdlib")]
    println!("cargo:rerun-if-env-changed=LOCAL_TDLIB_PATH");

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
