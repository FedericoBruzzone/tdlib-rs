// Copyright 2020 - developers of the `grammers` project.
// Copyright 2021 - developers of the `tdlib-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
mod generated;
mod observer;
mod tdjson;

pub use generated::{enums, functions, types};

use enums::Update;
use once_cell::sync::Lazy;
use serde_json::Value;
use std::sync::atomic::{AtomicU32, Ordering};

static EXTRA_COUNTER: AtomicU32 = AtomicU32::new(0);
static OBSERVER: Lazy<observer::Observer> = Lazy::new(observer::Observer::new);

/// Create a TdLib client returning its id. Note that to start receiving
/// updates for a client you need to send at least a request with it first.
pub fn create_client() -> i32 {
    tdjson::create_client()
}

/// Receive a single update or response from TdLib. If it's an update, it
/// returns a tuple with the `Update` and the associated `client_id`.
/// Note that to start receiving updates for a client you need to send
/// at least a request with it first.
pub fn receive() -> Option<(Update, i32)> {
    let response = tdjson::receive(2.0);
    if let Some(response_str) = response {
        let response: Value = serde_json::from_str(&response_str).unwrap();

        match response.get("@extra") {
            Some(_) => {
                OBSERVER.notify(response);
            }
            None => {
                let client_id = response["@client_id"].as_i64().unwrap() as i32;
                match serde_json::from_value(response) {
                    Ok(update) => {
                        return Some((update, client_id));
                    }
                    Err(e) => {
                        log::warn!(
                            "Received an unknown response: {}\nReason: {}",
                            response_str,
                            e
                        );
                    }
                }
            }
        }
    }

    None
}

pub(crate) async fn send_request(client_id: i32, mut request: Value) -> Value {
    let extra = EXTRA_COUNTER.fetch_add(1, Ordering::Relaxed);
    request["@extra"] = serde_json::to_value(extra).unwrap();

    let receiver = OBSERVER.subscribe(extra);
    tdjson::send(client_id, request.to_string());

    receiver.await.unwrap()
}
