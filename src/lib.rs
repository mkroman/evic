// Copyright 2016 evic Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::result;

pub mod error;
pub mod firmware;

pub type Result<T> = result::Result<T, error::Error>;

pub mod prelude {
    pub use firmware::Firmware;
}
