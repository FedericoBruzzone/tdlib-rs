/// The build module is used to build the project using the enabled features.
/// The features are correctly set when exactly one of the following features is enabled:
/// - `local-tdlib`
/// - `pkg-config`
/// - `download-tdlib`

#[allow(dead_code)]
#[cfg(not(any(feature = "docs", feature = "pkg-config")))]
const TDLIB_VERSION: &str = "1.8.29";
#[cfg(feature = "download-tdlib")]
const TDLIB_CARGO_PKG_VERSION: &str = "1.0.5";

// WARNING: This function is not used in the current version of the library.
// #[cfg(not(any(feature = "docs", feature = "pkg-config", feature = "download-tdlib")))]
// fn copy_local_tdlib() {
//     match std::env::var("LOCAL_TDLIB_PATH") {
//         Ok(tdlib_path) => {
//             let out_dir = std::env::var("OUT_DIR").unwrap();
//             let prefix = format!("{}/tdlib", out_dir);
//             copy_dir_all(std::path::Path::new(&tdlib_path), std::path::Path::new(&prefix)).unwrap();
//         }
//         Err(_) => {
//             panic!("The LOCAL_TDLIB_PATH env variable must be set to the path of the tdlib folder");
//         }
//     };
// }

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
/// - Linux x86_64
/// - Windows x86_64
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

    // Create the request
    let response = reqwest::blocking::get(&url).unwrap();

    // Check if the response status is successful
    if response.status().is_success() {
        // Create a file to write to
        let mut dest = std::fs::File::create(&zip_path).unwrap();

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

    let mut archive = zip::ZipArchive::new(std::fs::File::open(&zip_path).unwrap()).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = std::path::Path::new(&out_dir).join(file.name());

        if (*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p).unwrap();
                }
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
/// - `cargo:rustc-link-search=native=.../tdlib/bin` (only for Windows x86_64)
///
/// The `...` represents the `dest_path` or the `OUT_DIR` environment variable.
///
/// If the tdlib library is not found at the specified path, the function will panic.
///
/// The function will panic if the tdlib library is not found at the specified path.
fn generic_build(lib_path: Option<String>) {
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
    let include_dir = format!("{}/include", prefix);
    let lib_dir = format!("{}/lib", prefix);
    let mut_lib_path = {
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

    if !std::path::PathBuf::from(mut_lib_path.clone()).exists() {
        panic!("tdjson shared library not found at {}", mut_lib_path);
    }

    // This should be not necessary, but it is a workaround because windows does not find the
    // tdjson.dll using the commands below.
    // TODO: investigate and if it is a bug in `cargo` or `rustc`, open an issue to `cargo` to fix
    // this.
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        let bin_dir = format!(r"{}\bin", prefix);
        let cargo_bin = format!("{}/.cargo/bin", dirs::home_dir().unwrap().to_str().unwrap());

        let libcrypto3x64 = format!(r"{}\libcrypto-3-x64.dll", bin_dir);
        let libssl3x64 = format!(r"{}\libssl-3-x64.dll", bin_dir);
        let tdjson = format!(r"{}\tdjson.dll", bin_dir);
        let zlib1 = format!(r"{}\zlib1.dll", bin_dir);

        let cargo_libcrypto3x64 = format!(r"{}\libcrypto-3-x64.dll", cargo_bin);
        let cargo_libssl3x64 = format!(r"{}\libssl-3-x64.dll", cargo_bin);
        let cargo_tdjson = format!(r"{}\tdjson.dll", cargo_bin);
        let cargo_zlib1 = format!(r"{}\zlib1.dll", cargo_bin);

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
/// - `cargo:rustc-link-search=native=.../tdlib/bin` (only for Windows x86_64)
///
/// The `...` represents the `dest_path` or the `OUT_DIR` environment variable.
///
/// The function will download the tdlib library from the GitHub release page.
/// Using the `download-tdlib` feature, no system dependencies are required.
/// The OS and architecture currently supported are:
/// - Linux x86_64
/// - Windows x86_64
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
        if dest_path.is_some() {
            let out_dir = std::env::var("OUT_DIR").unwrap();
            let tdlib_dir = format!("{}/tdlib", &out_dir);
            let dest_path = dest_path.as_ref().unwrap();
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
/// - `cargo:rustc-link-search=native=.../tdlib/bin` (only for Windows x86_64)
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
