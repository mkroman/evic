// Copyright 2016 evic Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::io;
use std::fmt;
use std::error;

/// Error types.
#[derive(Debug)]
pub enum Error {
    /// An error originating from reading or writing to the underlying buffer.
    Io(io::Error),
    /// An error related to the provided firmware.
    Firmware(String),
    /// An error originating from the main application.
    CliError(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::Firmware(ref err) => write!(f, "Firmware error: {}", err),
            Error::CliError(ref err) => write!(f, "CLI error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(..) => "eVic IO error",
            Error::Firmware(..) => "eVic firmware error",
            Error::CliError(..) => "eVic CLI error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            _ => None
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}
