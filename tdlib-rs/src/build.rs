#[allow(dead_code)]
#[cfg(not(any(feature = "docs", feature = "pkg-config")))]
/// The version of the TDLib library.
const TDLIB_VERSION: &str = "1.8.29";
#[cfg(feature = "download-tdlib")]
const TDLIB_CARGO_PKG_VERSION: &str = "1.0.3";

pub fn check_features() {
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
}

pub fn set_rerun_if() {
    #[cfg(not(any(feature = "docs", feature = "pkg-config", feature = "download-tdlib")))]
    println!("cargo:rerun-if-env-changed=LOCAL_TDLIB_PATH");

    println!("cargo:rerun-if-changed=build.rs");
}

// You have to build the tdlib
// TODO: Try to change the .pc file
pub fn build_pkg_config() {
    #[cfg(not(feature = "docs"))]
    {
        // It requires the following variables to be set:
        // - export PKG_CONFIG_PATH=$HOME/lib/tdlib/lib/pkgconfig/:$PKG_CONFIG_PATH
        // - export LD_LIBRARY_PATH=$HOME/lib/tdlib/lib/:$LD_LIBRARY_PATH
        #[cfg(feature = "pkg-config")]
        system_deps::Config::new().probe().unwrap();
    }
}

#[cfg(feature = "download-tdlib")]
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

#[cfg(not(any(feature = "docs", feature = "pkg-config")))]
/// Build the project using the generic build configuration.
/// The current supported platforms are:
/// - Linux x86_64
/// - Windows x86_64
/// - MacOS x86_64
/// - MacOS aarch64
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
pub fn build_download_tdlib(dest_path: Option<String>) {
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

// #[cfg(not(any(feature = "docs", feature = "pkg-config", feature = "download-tdlib")))]
// /// Copy all the tdlib folder find in the LOCAL_TDLIB_PATH environment variable to the OUT_DIR/tdlib folder
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

#[cfg(not(any(feature = "docs", feature = "pkg-config", feature = "download-tdlib")))]
pub fn build_local_tdlib() {
    // copy_local_tdlib();
    let path = std::env::var("LOCAL_TDLIB_PATH").unwrap();
    generic_build(Some(path));
}
