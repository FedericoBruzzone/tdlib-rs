// Copyright 2021 - developers of the `tdlib-rs` project.
// Copyright 2020 - developers of the `grammers` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use std::env;
use std::fs::File;
use std::io::{self, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
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

fn main() -> std::io::Result<()> {
    // Prevent linking libraries to avoid documentation failure
    // #[cfg(not(feature = "dox"))]
    // system_deps::Config::new().probe().unwrap();

    // TOOD Get artifacts from github (lib folder and include folder)
    #[cfg(not(feature = "dox"))]
    {
        let tdlib_download_path = "/home/fcb/lib/tdlib";

        let out_dir = env::var("OUT_DIR").unwrap();
        let out_dir = Path::new(&out_dir);
        let _ = std::process::Command::new("cp")
            .args(&["-r", tdlib_download_path, out_dir.to_str().unwrap()])
            .output()
            .expect("failed to copy lib/tdlib to OUT_DIR");

        let prefix = format!("{}/tdlib", out_dir.to_str().unwrap());
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

    let definitions = load_tl("tl/api.tl")?;

    let mut file = BufWriter::new(File::create(
        Path::new(&env::var("OUT_DIR").unwrap()).join("generated.rs"),
    )?);

    generate_rust_code(&mut file, &definitions, cfg!(feature = "bots-only-api"))?;

    file.flush()?;
    Ok(())
}
