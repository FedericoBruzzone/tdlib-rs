# Compiling to Wasm

- Follow the [steps](https://github.com/tdlib/td/tree/master/example/web) in the official tdlib repo. If it shows some errors, it's fine, as long as you can find a `libtdjson_static.a` file in the `td/example/web/build/wasm/` directory.
- Run `export LOCAL_TDLIB_PATH=path/to/td/example/web/build/wasm/`. Now `tdjson.rs` will link to this version of Tdlib compiled specifically for Wasm
- Run `cargo build --target wasm32-wasi`
