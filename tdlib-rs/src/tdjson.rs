// Copyright 2020 - developers of the `grammers` project.
// Copyright 2021 - developers of the `tdlib-rs` project.
// Copyright 2024 - developers of the `tgt` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int};

#[link(name = "tdjson")]
extern "C" {
    fn td_create_client_id() -> c_int;
    fn td_send(client_id: c_int, request: *const c_char);
    fn td_receive(timeout: c_double) -> *const c_char;
}

pub(crate) fn create_client() -> i32 {
    unsafe { td_create_client_id() }
}

pub(crate) fn send(client_id: i32, request: String) {
    let cstring = CString::new(request).unwrap();
    unsafe { td_send(client_id, cstring.as_ptr()) }
}

pub(crate) fn receive(timeout: f64) -> Option<String> {
    unsafe {
        td_receive(timeout)
            .as_ref()
            .map(|response| CStr::from_ptr(response).to_string_lossy().into_owned())
    }
}
