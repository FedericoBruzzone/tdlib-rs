// cargo run -p tdlib-rs --example test_ci --features default
// cargo run -p tdlib-rs --example test_ci --features download-tdlib
// cargo run -p tdlib-rs --example test_ci --features pkg-config

#[tokio::main]
async fn main() {
    // Create the client object for testing
    let _client_id = tdlib_rs::create_client();

    // Exit 0
    std::process::exit(0);
}
