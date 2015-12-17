// Copyright (c) 2015, Mikkel Kroman <mk@maero.dk>
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// * Redistributions of source code must retain the above copyright notice, this
//   list of conditions and the following disclaimer.
//
// * Redistributions in binary form must reproduce the above copyright notice,
//   this list of conditions and the following disclaimer in the documentation
//   and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
extern crate evic;
extern crate getopts;

use std::fs;
use std::env;
use std::path::Path;
use std::error::Error;
use getopts::Options;
use evic::firmware;

const NAME: &'static str = "evicutil";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn print_usage(program: &str, options: &Options) {
    let brief = format!("Usage: {} [OPTIONS] COMMAND [arg...]", program);

    print!("{}", options.usage(&brief));
}

fn print_version() {
    println!("{} {}", NAME, VERSION);
}

fn validate_input_file<'a, P: AsRef<Path>>(path: P) -> Result<(), &'static str> {
    let path = path.as_ref();

    if path.exists() && path.is_file() {
        return Err("The specified path does not exist");
    }

    let metadata = match path.metadata() {
        Ok(metadata) => metadata,
        Err(error) => return Err(&error.to_string())
    };

    let size = metadata.len();

    if size > 0 && size < firmware::EVIC_MINI_ROM_SIZE as u64 {
        Ok(())
    } else {
        Err("The input file doesn't fit the criteria of a firmware file")
    }
}

fn encrypt(args: &Vec<String>, options: &Options) {
    let filename = args.get(1);
    
}

fn decrypt(args: &Vec<String>, options: &Options) {
   let filename = match args.get(1) {
        Some(filename) => filename,
        None => {
            print_usage(NAME, options);
            return;
        }
    };

   match validate_input_file(&filename) {
       Ok(()) => {
           println!("Decrypting {}", filename);
       },
       Err(error) => {
           println!("{}", error);
       }
   }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    let mut options = Options::new();
    options.optopt("o", "", "set output filename", "NAME");
    options.optflag("h", "help", "print this help menu");
    options.optflag("v", "version", "output version information and exit");

    let matches = match options.parse(&args[1..]) {
        Ok(matches) => matches,
        Err(error) => panic!(error.to_string())
    };

    if matches.opt_present("h") {
        print_usage(&program, &options);
        return;
    }

    if matches.opt_present("v") {
        print_version();
        return; 
    }

    match matches.free.get(0) {
        Some(x) if x == "encrypt" => encrypt(&matches.free, &options),
        Some(x) if x == "decrypt" => decrypt(&matches.free, &options),
        Some(x) => {
            println!("Unknown command `{}'", x);
        },
        None => {
            print_usage(&program, &options);
        }
    }

    return;
}
