# tdlib-rs

[![CI Linux](https://github.com/FedericoBruzzone/tdlib-rs/actions/workflows/ci-linux.yml/badge.svg)](https://github.com/FedericoBruzzone/tdlib-rs/actions/workflows/ci-linux.yml)
[![CI Windows](https://github.com/FedericoBruzzone/tdlib-rs/actions/workflows/ci-windows.yml/badge.svg)](https://github.com/FedericoBruzzone/tdlib-rs/actions/workflows/ci-windows.yml)
[![CI macOS](https://github.com/FedericoBruzzone/tdlib-rs/actions/workflows/ci-macos.yml/badge.svg)](https://github.com/FedericoBruzzone/tdlib-rs/actions/workflows/ci-macos.yml)

A Rust wrapper around the Telegram Database library. It includes a generator to automatically generate the types and functions from the TDLib's [Type Language](https://core.telegram.org/mtproto/TL) file.

## Why this fork?

This is an improved version of the [tdlib-rs](https://github.com/paper-plane-developers/tdlib-rs) library, with the following additional features:

1. It is cross-platform, it works on Windows, Linux and MacOS.
2. Not required `pkg-config` to build the library and associated exported variables.
3. Not required `tdlib` to be compiled and installed on the system.
4. It is possible to download the `tdlib` library from the GitHub releases.

## Information

We provide a precompiled version of the library for the supported platforms:

- Linux (x86_64)
- Windows (x86_64)
- macOS Intel (x86_64)
- macOS Apple Silicon (arm64)

We compile it in the CI and we upload the artifacts to the GitHub releases, so we can download it and use to build this library.

It's mainly created for using it in the [tgt](https://github.com/FedericoBruzzone/tgt) client, but it should work also for any other Rust project.

Current supported TDLib version: [1.8.19](https://github.com/tdlib/td/commit/2589c3fd46925f5d57e4ec79233cd1bd0f5d0c09).

## Cargo features

### default

By default the library require you to have the `tdlib` (version 1.8.19) compiled and installed on your system, and the following variables exported, for example in the `.bashrc` file:

```sh
# The path to the tdlib folder
export LOCAL_TDLIB_PATH=$HOME/lib/tdlib
```

### bots-only-api

This feature enable the generation of the functions only used by Telegram bots.

### download-tdlib

If you don't want to compile and intall the `tdlib` on your system manually, you should enable the `download-tdlib` feature in the `Cargo.toml` file:

```toml
[dependencies]
tdlib = { version = "1.0.0", features = [ "download-tdlib" ] }
```

remember to check in the [information section](#information) if your platform is supported.

### pkg-config

If you want to use the `pkg-config` to build this library, you should enable the `pkg-config` feature in the `Cargo.toml` file:

```toml
[dependencies]
tdlib = { version = "1.0.0", features = [ "pkg-config" ] }
```

remember to have the `tdlib` (version 1.8.19) installed on your system, and the following variables exported, for example in the `.bashrc` file:

```sh
# pkg-config configuration
export PKG_CONFIG_PATH=$HOME/lib/tdlib/lib/pkgconfig/:$PKG_CONFIG_PATH

# dynmic linker configuration
export LD_LIBRARY_PATH=$HOME/lib/tdlib/lib/:$LD_LIBRARY_PATH
```

### docs

This feature skip the linking of the library and only generate the code of `generated.rs`.
Is used only for testing.

## License

This repository are licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE][github-license-apache] or <http://www.apache.org/licenses/LICENSE-2.0>)

- MIT license ([LICENSE-MIT][github-license-mit] or <http://opensource.org/licenses/MIT>)

at your option.

Please review the license file provided in the repository for more information regarding the terms and conditions of the license.

## Contact

- Email:
  - [federico.bruzzone.i@gmail.com]
  - [federico.bruzzone@studenti.unimi.it]
  - [andrea.longoni3@studenti.unimi.it]
- GitHub:
  - [FedericoBruzzone](https://github.com/FedericoBruzzone)
  - [Andreal2000](https://github.com/Andreal2000)

## Credits

- [grammers](https://github.com/Lonami/grammers): the `tdlib-tl-gen` and `tdlib-tl-parser` projects are forks of the `grammers-tl-gen` and `grammers-tl-parser` projects.
- [rust-tdlib](https://github.com/aCLr/rust-tdlib): for inspiration about some client code.
- [tdlib-rs](https://github.com/paper-plane-developers/tdlib-rs): for inspiration about the generator code.
