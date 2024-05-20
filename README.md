# tdlib-rs

A Rust wrapper around the Telegram Database library. It includes a generator to automatically generate the types and functions from the TDLib's [Type Language](https://core.telegram.org/mtproto/TL) file.

## Why this fork?

This is an improved version of the [tdlib-rs](https://github.com/paper-plane-developers/tdlib-rs) library, with the following features:
1. It is cross-platform, it should work on Windows, Linux and MacOS.
2. Not required `pkg-config` to build the library and associated exported variables.
3. Not required `tdlib` to be compiled and installed on the system.

## Information

We provide a precompiled version of the library for the supported platforms:
- Linux (x86_64)
- Windows (x86_64)
- MacOS Intel (x86_64)
- MacOS Apple Silicon (arm64)

We compile it in the CI and we upload the artifacts to the GitHub releases, so we can download it and use to build this library.

It's mainly created for using it in the [tgt](https://github.com/FedericoBruzzone/tgt) client, but it should work also for any other Rust project.

Current supported TDLib version: [1.8.19](https://github.com/tdlib/td/commit/2589c3fd46925f5d57e4ec79233cd1bd0f5d0c09).

### pkg-config support

If you want to use the `pkg-config` support, you should enable the `pkg-config` feature in the `Cargo.toml` file:

```toml
[dependencies]
tdlib = { version = "x", features = ["pkg-config"] }
```

remember to have the `tdlib` (version 1.8.19) installed on your system, and the following variables exported, for example in the `.bashrc` file:

```sh
# pkg-config configuration
export PKG_CONFIG_PATH=$HOME/lib/tdlib/lib/pkgconfig/:$PKG_CONFIG_PATH

# dynmic linker configuration
export LD_LIBRARY_PATH=$HOME/lib/tdlib/lib/:$LD_LIBRARY_PATH
```

## License

This repository are licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE][github-license-apache] or http://www.apache.org/licenses/LICENSE-2.0)

* MIT license ([LICENSE-MIT][github-license-mit] or http://opensource.org/licenses/MIT)

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
