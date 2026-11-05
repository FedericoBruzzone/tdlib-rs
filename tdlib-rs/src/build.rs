//! The build module is used to build the project using the enabled features.
//! The features are correctly set when exactly one of the following features is enabled:
//! - `local-tdlib`
//! - `pkg-config`
//! - `download-tdlib`

#[allow(dead_code)]
#[cfg(not(any(feature = "docs", feature = "pkg-config")))]
const TDLIB_VERSION: &str = "1.8.61";
#[cfg(feature = "download-tdlib")]
const TDLIB_CARGO_PKG_VERSION: &str = "1.4.0";

#[cfg(feature = "download-tdlib")]
/// Copy all files from a directory to another.
/// It assumes that the source directory exists.
/// If the destination directory does not exist, it will be created.
fn copy_dir_all(
    src: impl AsRef<std::path::Path>,
    dst: impl AsRef<std::path::Path>,
) -> std::io::Result<()> {
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

#[cfg(feature = "download-tdlib")]
/// Download the tdlib library from the GitHub release page.
/// The function will download the tdlib library from the GitHub release page, and extract the
/// files in the OUT_DIR/tdlib folder.
/// The OUT_DIR environment variable is set by Cargo and points to the target directory.
/// The OS and architecture currently supported are:
/// - Android x86_64
/// - Android aarch64
/// - Linux x86_64
/// - Linux aarch64
/// - Windows x86_64
/// - Windows aarch64
/// - MacOS x86_64
/// - MacOS aarch64
///
/// If the OS or architecture is not supported, the function will panic.
fn download_tdlib() {
    let base_url = "https://github.com/FedericoBruzzone/tdlib-rs/releases/download";
    let url = format!(
        "{}/v{}/tdlib-{}-{}-{}.zip",
        base_url,
        TDLIB_CARGO_PKG_VERSION,
        TDLIB_VERSION,
        std::env::var("CARGO_CFG_TARGET_OS").unwrap(),
        std::env::var("CARGO_CFG_TARGET_ARCH").unwrap(),
    );

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let tdlib_dir = format!("{}/tdlib", &out_dir);
    let zip_path = format!("{}.zip", &tdlib_dir);

    // Download a prebuilt tdlib archive using a blocking HTTP client.
    let response = ureq::get(&url).call();

    let mut response = match response {
        Ok(response) => response,
        Err(err) => {
            panic!(
                "[{}] Failed to download file: {}\n{}\n{}",
                "Your OS or architecture may be unsupported.",
                "Please try using the `pkg-config` or `local-tdlib` features.",
                err,
                &url
            )
        }
    };

    // Create a file to write to
    let mut dest = std::fs::File::create(&zip_path).unwrap();
    let mut response_reader = response.body_mut().as_reader();
    std::io::copy(&mut response_reader, &mut dest).unwrap();

    let mut archive = zip::ZipArchive::new(std::fs::File::open(&zip_path).unwrap()).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        // zip 9: `name()` returns a result and is not a safe extract path.
        let outpath = std::path::Path::new(&out_dir).join(
            file.enclosed_name()
                .expect("TDLib archive contains an invalid entry path"),
        );

        if file.is_dir() {
            std::fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent()
                && !p.exists()
            {
                std::fs::create_dir_all(p).unwrap();
            }
            let mut outfile = std::fs::File::create(&outpath).unwrap();
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

#[cfg(any(feature = "download-tdlib", feature = "local-tdlib"))]
/// Build the project using the `download-tdlib` or `local-tdlib` feature.
/// # Arguments
/// - `lib_path`: The path where the tdlib library is located. If `None`, the path will be the `OUT_DIR` environment variable.
///
/// The function will pass to the `rustc` the following flags:
/// - `cargo:rustc-link-search=native=.../tdlib/lib`
/// - `cargo:include=.../tdlib/include`
/// - `cargo:rustc-link-lib=dylib=tdjson`
/// - `cargo:rustc-link-arg=-Wl,-rpath,.../tdlib/lib`
/// - `cargo:rustc-link-search=native=.../tdlib/bin` (only for Windows)
///
/// The `...` represents the `dest_path` or the `OUT_DIR` environment variable.
///
/// If the tdlib library is not found at the specified path, the function will panic.
///
/// The function will panic if the tdlib library is not found at the specified path.
fn generic_build(lib_path: Option<String>) {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let correct_lib_path: String;
    match lib_path {
        Some(lib_path) => {
            if lib_path.ends_with('/') || lib_path.ends_with('\\') {
                correct_lib_path = lib_path[..lib_path.len() - 1].to_string();
            } else {
                correct_lib_path = lib_path.to_string();
            }
        }
        None => {
            correct_lib_path = format!("{}/tdlib", std::env::var("OUT_DIR").unwrap());
        }
    }
    let prefix = correct_lib_path.to_string();
    let include_dir = format!("{prefix}/include");
    let lib_dir = format!("{prefix}/lib");
    #[cfg(not(feature = "static"))]
    let dynamic_lib_path = match target_os.as_str() {
        "android" => format!("{lib_dir}/libtdjson.so"),
        "linux" => format!("{lib_dir}/libtdjson.so.{TDLIB_VERSION}"),
        "macos" => format!("{lib_dir}/libtdjson.{TDLIB_VERSION}.dylib"),
        "windows" => format!(r"{lib_dir}\tdjson.lib"),
        _ => panic!("Unsupported target OS: {target_os}"),
    };

    #[cfg(feature = "static")]
    let static_libs = [
        "tdactor",
        "tdapi",
        "tdclient",
        "tdcore",
        "tddb",
        "tde2e",
        "tdjson_private",
        "tdjson_static",
        "tdmtproto",
        "tdnet",
        "tdsqlite",
        "tdutils",
    ];

    #[cfg(feature = "static")]
    let static_libs_external = if target_os == "windows" {
        ["libssl", "libcrypto", "zlib"]
    } else {
        ["ssl", "crypto", "z"]
    };

    #[cfg(feature = "static")]
    let all_static_libs: Vec<String> = static_libs
        .iter()
        .map(|name| name.to_string())
        .chain(static_libs_external.iter().map(|name| name.to_string()))
        .collect();

    #[cfg(feature = "static")]
    let missing_static_libs: Vec<String> = all_static_libs
        .iter()
        .filter_map(|name| {
            let path = if target_os == "windows" {
                format!(r"{lib_dir}\{name}.lib")
            } else {
                format!("{lib_dir}/lib{name}.a")
            };

            if std::path::PathBuf::from(path.clone()).exists() {
                None
            } else {
                Some(path)
            }
        })
        .collect();

    #[cfg(feature = "static")]
    if !missing_static_libs.is_empty() {
        panic!(
            "required TDLib static libraries not found: {}",
            missing_static_libs.join(", ")
        );
    }

    #[cfg(not(feature = "static"))]
    if !std::path::PathBuf::from(dynamic_lib_path.clone()).exists() {
        panic!("tdjson shared library not found at {dynamic_lib_path}");
    }

    // This should be not necessary, but it is a workaround because windows does not find the
    // tdjson.dll using the commands below.
    // TODO: investigate and if it is a bug in `cargo` or `rustc`, open an issue to `cargo` to fix
    // this.
    #[cfg(not(feature = "static"))]
    if target_os == "windows" {
        let bin_dir = format!(r"{prefix}\bin");
        let cargo_bin = format!("{}/.cargo/bin", dirs::home_dir().unwrap().to_str().unwrap());

        let libcrypto3x64 = format!(r"{bin_dir}\libcrypto-3-x64.dll");
        let libssl3x64 = format!(r"{bin_dir}\libssl-3-x64.dll");
        let tdjson = format!(r"{bin_dir}\tdjson.dll");
        let zlib1 = format!(r"{bin_dir}\zlib1.dll");

        let cargo_libcrypto3x64 = format!(r"{cargo_bin}\libcrypto-3-x64.dll");
        let cargo_libssl3x64 = format!(r"{cargo_bin}\libssl-3-x64.dll");
        let cargo_tdjson = format!(r"{cargo_bin}\tdjson.dll");
        let cargo_zlib1 = format!(r"{cargo_bin}\zlib1.dll");

        // Delete the files if they exist
        let _ = std::fs::remove_file(&cargo_libcrypto3x64);
        let _ = std::fs::remove_file(&cargo_libssl3x64);
        let _ = std::fs::remove_file(&cargo_tdjson);
        let _ = std::fs::remove_file(&cargo_zlib1);

        // Move all files to cargo_bin
        let _ = std::fs::copy(libcrypto3x64.clone(), cargo_libcrypto3x64.clone());
        let _ = std::fs::copy(libssl3x64.clone(), cargo_libssl3x64.clone());
        let _ = std::fs::copy(tdjson.clone(), cargo_tdjson.clone());
        let _ = std::fs::copy(zlib1.clone(), cargo_zlib1.clone());
    }

    #[cfg(not(feature = "static"))]
    if target_os == "windows" {
        let bin_dir = format!(r"{prefix}\bin");
        println!("cargo:rustc-link-search=native={bin_dir}");
    }

    println!("cargo:rustc-link-search=native={lib_dir}");
    println!("cargo:include={include_dir}");
    #[cfg(feature = "static")]
    for link_name in &all_static_libs {
        println!("cargo:rustc-link-lib=static={link_name}");
    }
    #[cfg(feature = "static")]
    {
        // Link C++ standard library for static tdlib
        if target_os == "linux" || target_os == "macos" {
            println!("cargo:rustc-link-lib=c++");
            println!("cargo:rustc-link-lib=c++abi");
        } else if target_os == "android" {
            println!("cargo:rustc-link-lib=static=c++_static");
        } else if target_os == "windows" {
            // Windows system libraries required by TDLib
            println!("cargo:rustc-link-lib=psapi");
            println!("cargo:rustc-link-lib=Normaliz");
            println!("cargo:rustc-link-lib=Crypt32");
        } else {
            panic!("Unsupported target OS: {target_os}");
        }
    }
    #[cfg(not(feature = "static"))]
    println!("cargo:rustc-link-lib=dylib=tdjson");
    #[cfg(not(feature = "static"))]
    println!("cargo:rustc-link-arg=-Wl,-rpath,{lib_dir}");
}

/// Check if the features are correctly set.
/// The features are correctly set when exactly one of the following features is enabled:
/// - `local-tdlib`
/// - `pkg-config`
/// - `download-tdlib`
/// - `docs` (only for tdlib documentation)
///
/// The following features cannot be enabled at the same time:
/// - `docs` and `pkg-config`
/// - `docs` and `download-tdlib`
/// - `docs` and `local-tdlib`
/// - `pkg-config` and `local-tdlib`
/// - `pkg-config` and `download-tdlib`
/// - `local-tdlib` and `download-tdlib`
///
/// If the features are not correctly set, the function will generate a compile error
pub fn check_features() {
    // #[cfg(not(any(feature = "docs", feature = "local-tdlib", feature = "pkg-config", feature = "download-tdlib")))]
    // println!("cargo:warning=No features enabled, you must enable at least one of the following features: docs, local-tdlib, pkg-config, download-tdlib");
    // compile_error!("You must enable at least one of the following features: docs, local-tdlib, pkg-config, download-tdlib");

    #[cfg(all(feature = "docs", feature = "pkg-config"))]
    compile_error!(
        "feature \"docs\" and feature \"pkg-config\" cannot be enabled at the same time"
    );
    #[cfg(all(feature = "docs", feature = "download-tdlib"))]
    compile_error!(
        "feature \"docs\" and feature \"download-tdlib\" cannot be enabled at the same time"
    );
    #[cfg(all(feature = "docs", feature = "local-tdlib"))]
    compile_error!(
        "feature \"docs\" and feature \"local-tdlib\" cannot be enabled at the same time"
    );

    #[cfg(all(feature = "pkg-config", feature = "local-tdlib"))]
    compile_error!(
        "feature \"pkg-config\" and feature \"local-tdlib\" cannot be enabled at the same time"
    );
    #[cfg(all(feature = "pkg-config", feature = "download-tdlib"))]
    compile_error!(
        "feature \"pkg-config\" and feature \"download-tdlib\" cannot be enabled at the same time"
    );
    #[cfg(all(feature = "static", feature = "pkg-config"))]
    compile_error!(
        "feature \"static\" and feature \"pkg-config\" cannot be enabled at the same time"
    );
    #[cfg(all(feature = "local-tdlib", feature = "download-tdlib"))]
    compile_error!(
        "feature \"local-tdlib\" and feature \"download-tdlib\" cannot be enabled at the same time"
    );
}

/// Set the `rerun-if-changed` and `rerun-if-env-changed` flags for the build script.
/// The `rerun-if-changed` flag is set for the `build.rs` file.
/// The `rerun-if-env-changed` flag is set for the `LOCAL_TDLIB_PATH` environment variable.
pub fn set_rerun_if() {
    #[cfg(feature = "local-tdlib")]
    println!("cargo:rerun-if-env-changed=LOCAL_TDLIB_PATH");

    println!("cargo:rerun-if-changed=build.rs");
}

#[cfg(any(feature = "pkg-config", feature = "docs"))]
#[allow(clippy::needless_doctest_main)]
/// Build the project using the `pkg-config` feature.
/// Using the `pkg-config` feature, the function will probe the system dependencies.
/// It means that the function assumes that the tdlib library is compiled in the system.
/// It requires the following variables to be set:
/// - `PKG_CONFIG_PATH=$HOME/lib/tdlib/lib/pkgconfig/:$PKG_CONFIG_PATH`
/// - `LD_LIBRARY_PATH=$HOME/lib/tdlib/lib/:$LD_LIBRARY_PATH`
///
/// If the variables are not set, the function will panic.
///
/// # Example
/// Cargo.toml:
/// ```toml
/// [dependencies]
/// tdlib = { version = "...", features = ["pkg-config"] }
/// ```
///
/// build.rs:
/// ```rust
/// fn main() {
///   tdlib_rs::build::check_features();
///   tdlib_rs::build::set_rerun_if();
///   tdlib_rs::build::build_pkg_config();
///   // Other build configurations
///   // ...
/// }
/// ```
pub fn build_pkg_config() {
    #[cfg(not(feature = "docs"))]
    {
        system_deps::Config::new().probe().unwrap();
    }
}

#[cfg(any(feature = "download-tdlib", feature = "docs"))]
#[allow(clippy::needless_doctest_main)]
#[allow(unused_variables)]
/// Build the project using the `download-tdlib` feature.
///
/// # Arguments
/// - `dest_path`: The destination path where the tdlib library will be copied. If `None`, the path will be the `OUT_DIR` environment variable.
///
/// Note that this function will pass to the `rustc` the following flags:
/// - `cargo:rustc-link-search=native=.../tdlib/lib`
/// - `cargo:include=.../tdlib/include`
/// - `cargo:rustc-link-lib=dylib=tdjson`
/// - `cargo:rustc-link-arg=-Wl,-rpath,.../tdlib/lib`
/// - `cargo:rustc-link-search=native=.../tdlib/bin` (only for Windows)
///
/// The `...` represents the `dest_path` or the `OUT_DIR` environment variable.
///
/// The function will download the tdlib library from the GitHub release page.
/// Using the `download-tdlib` feature, no system dependencies are required.
/// The OS and architecture currently supported are:
/// - Android x86_64
/// - Android aarch64
/// - Linux x86_64
/// - Linux aarch64
/// - Windows x86_64
/// - Windows aarch64
/// - MacOS x86_64
/// - MacOS aarch64
///
/// If the OS or architecture is not supported, the function will panic.
///
/// # Example
/// Cargo.toml:
/// ```toml
/// [dependencies]
/// tdlib = { version = "...", features = ["download-tdlib"] }
///
/// [build-dependencies]
/// tdlib = { version = "...", features = [ "download-tdlib" ] }
/// ```
///
/// build.rs:
/// ```rust
/// fn main() {
///   tdlib_rs::build::check_features();
///   tdlib_rs::build::set_rerun_if();
///   tdlib_rs::build::build_download_tdlib(None);
///   // Other build configurations
///   // ...
/// }
/// ```
pub fn build_download_tdlib(dest_path: Option<String>) {
    #[cfg(not(feature = "docs"))]
    {
        download_tdlib();
        if let Some(dest_path) = &dest_path {
            let out_dir = std::env::var("OUT_DIR").unwrap();
            let tdlib_dir = format!("{}/tdlib", &out_dir);
            copy_dir_all(
                std::path::Path::new(&tdlib_dir),
                std::path::Path::new(&dest_path),
            )
            .unwrap();
        }
        generic_build(dest_path);
    }
}
#[cfg(any(feature = "local-tdlib", feature = "docs"))]
#[allow(clippy::needless_doctest_main)]
/// Build the project using the `local-tdlib` feature.
/// Using the `local-tdlib` feature, the function will copy the tdlib library from the
/// `LOCAL_TDLIB_PATH` environment variable.
/// The tdlib folder must contain the `lib` and `include` folders.
/// You can directly download the tdlib library from the [TDLib Release GitHub page](https://github.com/FedericoBruzzone/tdlib-rs/releases).
///
/// The `LOCAL_TDLIB_PATH` environment variable must be set to the path of the tdlib folder.
///
/// The function will pass to the `rustc` the following flags:
/// - `cargo:rustc-link-search=native=.../tdlib/lib`
/// - `cargo:include=.../tdlib/include`
/// - `cargo:rustc-link-lib=dylib=tdjson`
/// - `cargo:rustc-link-arg=-Wl,-rpath,.../tdlib/lib`
/// - `cargo:rustc-link-search=native=.../tdlib/bin` (only for Windows)
///
/// The `...` represents the `LOCAL_TDLIB_PATH` environment variable.
///
/// If the `LOCAL_TDLIB_PATH` environment variable is not set, the function will panic.
///
/// # Example
/// Cargo.toml:
/// ```toml
/// [dependencies]
/// tdlib = { version = "...", features = ["local-tdlib"] }
///
/// [build-dependencies]
/// tdlib = { version = "...", features = [ "download-tdlib" ] }
/// ```
///
/// build.rs:
/// ```rust
/// fn main() {
///   tdlib_rs::build::check_features();
///   tdlib_rs::build::set_rerun_if();
///   tdlib_rs::build::build_local_tdlib();
///   // Other build configurations
///   // ...
/// }
/// ```
pub fn build_local_tdlib() {
    #[cfg(not(feature = "docs"))]
    {
        // copy_local_tdlib();
        let path = std::env::var("LOCAL_TDLIB_PATH").unwrap();
        generic_build(Some(path));
    }
}

#[allow(clippy::needless_doctest_main)]
/// Build the project using the enabled features.
///
/// # Arguments
/// - `dest_path`: The destination path where the tdlib library will be copied. If `None`, the path
///   will be the `OUT_DIR` environment variable. This argument is used only when the
///   `download-tdlib` feature is enabled.
///
/// The function will check if the features are correctly set.
/// The function will set the `rerun-if-changed` and `rerun-if-env-changed` flags for the build
/// script.
/// The function will build the project using the enabled feature.
///
/// # Example
/// Cargo.toml:
/// ```toml
/// [dependencies]
/// tdlib = { version = "...", features = ["download-tdlib"] }
///
/// [build-dependencies]
/// tdlib = { version = "...", features = [ "download-tdlib" ] }
/// ```
///
/// build.rs:
/// ```rust
/// fn main() {
///   tdlib_rs::build::build(None);
///   // Other build configurations
///   // ...
/// }
/// ```
pub fn build(_dest_path: Option<String>) {
    check_features();
    set_rerun_if();

    #[cfg(feature = "pkg-config")]
    build_pkg_config();
    #[cfg(feature = "download-tdlib")]
    build_download_tdlib(_dest_path);
    #[cfg(feature = "local-tdlib")]
    build_local_tdlib();
}
