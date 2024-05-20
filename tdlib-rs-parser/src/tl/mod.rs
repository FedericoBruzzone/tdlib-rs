// Copyright 2020 - developers of the `grammers` project.
// Copyright 2022 - developers of the `tdlib-rs` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This module contains all the different structures representing the
//! various terms of the [Type Language].
//!
//! [Type Language]: https://core.telegram.org/mtproto/TL
mod category;
mod definition;
mod parameter;
mod ty;

pub use category::Category;
pub use definition::Definition;
pub use parameter::Parameter;
pub use ty::Type;
