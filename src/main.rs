// Copyright 2016 evic Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
extern crate evic;
extern crate getopts;

use std::fs;
use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use getopts::Options;
use evic::Result;
use evic::firmware::Firmware;

const NAME: &'static str = "evicutil";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

enum Mode {
    Encrypt,
    Decrypt,
    Version,
    Help,
}

fn print_usage(usage: &str) {
    print!("{}", usage);
}

fn print_version() {
    println!("{} {}", NAME, VERSION);
}

// Take a provided input path and return a suffixed, if necessary, output path.
//
// When the input path is None, it will return a path where the filename is suffixed with
// `suffix`.
fn suffixed_file_path<P>(path: P, suffix: &str) -> PathBuf
    where P: AsRef<Path> {
    let mut buf = path.as_ref().to_path_buf();
    let mut file_name = OsString::from(buf.file_stem().unwrap_or("firmware".as_ref()));

    file_name.push(suffix);
    file_name.push(".");
    file_name.push(buf.extension().unwrap_or("bin".as_ref()));

    buf.set_file_name(file_name);
    buf
}

fn encrypt<P: AsRef<Path>, T: AsRef<Path>>(path: P, output_path: T) -> Result<()> {
   let mut file = try!(fs::File::open(path.as_ref()));
   let firmware = try!(Firmware::decrypt(&mut file));
   let mut output_file = try!(fs::File::create(output_path.as_ref()));

   firmware.save(&mut output_file).unwrap();

   Ok(())
}

fn decrypt<P: AsRef<Path>, T: AsRef<Path>>(path: P, output_path: T) -> Result<()> {
   let mut file = try!(fs::File::open(path.as_ref()));
   let firmware = try!(Firmware::decrypt(&mut file));
   let mut output_file = try!(fs::File::create(output_path.as_ref()));

   firmware.save(&mut output_file).unwrap();

   Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    let mut options = Options::new();
    options.optopt("o", "", "set output filename", "NAME");
    options.optflag("h", "help", "print this help menu");
    options.optflag("v", "version", "output version information and exit");

    let matches = options.parse(&args[1..]).unwrap();

    let mode = if matches.opt_present("h") {
        Mode::Help
    } else if matches.opt_present("v") {
        Mode::Version
    } else if let Some(command) = matches.free.get(0) {
        if command == "encrypt" {
            Mode::Encrypt
        } else if command == "decrypt" {
            Mode::Decrypt
        } else {
            println!("Unknown command `{}'", command);
            Mode::Help
        }
    } else {
        Mode::Help
    };

    match mode {
        Mode::Encrypt => {
            if let Some(ref path) = matches.free.get(1) {
                let output_path = match matches.opt_str("o") {
                    Some(ref path) => PathBuf::from(path),
                    None => suffixed_file_path(path, "_encrypted")
                };

                encrypt(path, &output_path).unwrap();
            } else {
                println!("Please supply a file path.");
            }
        },
        Mode::Decrypt => {
            if let Some(ref path) = matches.free.get(1) {
                let output_path = match matches.opt_str("o") {
                    Some(ref path) => PathBuf::from(path),
                    None => suffixed_file_path(path, "_decrypted")
                };

                decrypt(path, &output_path).unwrap();
            } else {
                println!("Please supply a file path.");
            }
        },
        Mode::Version => {
            print_version();
        }
        Mode::Help => {
            let brief = format!("Usage: {} [OPTIONS] COMMAND [arg...]", program);
            let usage = options.usage(&brief);

            print_usage(&usage);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_should_give_suffixed_output_path() {
        use std::path::Path;

        let path = Path::new("test.bin");
        let output_path = super::suffixed_file_path(path, "_decrypted");

        assert_eq!(Path::new("test_decrypted.bin"), output_path.as_path());
    }

    #[test]
    fn it_should_give_suffixed_output_path_despite_no_extension() {
        use std::path::Path;

        let path = Path::new("test");
        let output_path = super::suffixed_file_path(path, "_decrypted");

        assert_eq!(Path::new("test_decrypted.bin"), output_path.as_path());
    }
}
