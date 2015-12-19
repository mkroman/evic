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

use std::fs::File;
use std::io::{Read, Write, Seek, SeekFrom};
use super::Result;
use super::error::Error;

/// The capacity of the ROM in the eVic VTC Mini.
pub const EVIC_MINI_ROM_SIZE: usize = 120 * 1024;

#[derive(Debug)]
pub struct Firmware {
    buffer: Vec<u8>,
}

impl Firmware {
    /// Initializes a new firmware structure with the capacity equal to the maximum
    /// ROM capacity.
    fn new() -> Firmware {
        Firmware {
            buffer: Vec::with_capacity(EVIC_MINI_ROM_SIZE)
        }
    }

    /// Decrypts the firmware from the provided reader.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Cursor;
    /// use evic::firmware::Firmware;
    ///
    /// let buffer = [0; 1024]; // Pretend this is a buffer holding encrypted firmware.
    /// let firmware = Firmware::decrypt(&mut Cursor::new(buffer.as_ref())).unwrap();
    /// ```
    pub fn decrypt<R: Read + Seek + ?Sized>(reader: &mut R) -> Result<Firmware> {
        let mut firmware = Firmware::new();

        // Attempt to get the size so we can check whether it exceeds the capacity of the device.
        let size = try!(reader.seek(SeekFrom::End(0)));

        if size > EVIC_MINI_ROM_SIZE as u64 {
            return Err(Error::Firmware(format!("the firmware image exceeds the devices ROM capacity")));
        }

        // Go back to the beginning of the buffer before we read it.
        try!(reader.seek(SeekFrom::Start(0)));
        try!(reader.read_to_end(&mut firmware.buffer));

        // Decrypt the firmware.
        for (i, x) in firmware.buffer.iter_mut().enumerate() {
            *x ^= (i as u64 + 408376 + size - size / 408376) as u8;
        }

        Ok(firmware)
    }

    /// Writes the contents of the firmware buffer to the provided writer.
    pub fn save<W: Write + ?Sized>(&self, writer: &mut W) -> Result<()> {
        try!(writer.write_all(&self.buffer[..]));

        Ok(())
    }
}

/// Loads a firmware file from the specified path and decrypts it in memory.
pub fn load(path: &str) -> Result<Firmware> {
    let mut file = try!(File::open(path));
    let firmware = try!(Firmware::decrypt(&mut file));

    Ok(firmware)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_decrypts() {
        use std::io::Cursor;

        let encrypted_firmware_data: [u8; 100] = [0xd0, 0xf2, 0xec, 0xfa, 0xcd, 0x81, 0xcb, 0xd3,
                                                  0xd7, 0xd0, 0xcb, 0x87, 0xcc, 0xc6, 0xc6, 0xc4,
                                                  0xde, 0x8d, 0xdd, 0xc6, 0xc4, 0x91, 0xd3, 0xde,
                                                  0xd1, 0xc1, 0x9a, 0x97, 0xdb, 0xd6, 0xd4, 0xc8,
                                                  0xd9, 0xde, 0xca, 0xda, 0xb4, 0xb4, 0xb0, 0xe3,
                                                  0xa5, 0xa1, 0xaf, 0xb7, 0xa1, 0xba, 0xa9, 0xa2,
                                                  0xa2, 0xaa, 0xee, 0xaa, 0xbc, 0xb8, 0xa6, 0xfd,
                                                  0xf4, 0x85, 0xa4, 0xb8, 0xb1, 0xb7, 0xfa, 0xbd,
                                                  0xbd, 0xbe, 0xb7, 0xb3, 0x89, 0x92, 0x8b, 0x90,
                                                  0xc4, 0x97, 0x8e, 0x88, 0x86, 0x8a, 0x9f, 0x98,
                                                  0xcc, 0x9d, 0x82, 0x8e, 0x93, 0x94, 0x80, 0x92,
                                                  0x80, 0xd5, 0x95, 0x85, 0x99, 0x8a, 0xda, 0x9a,
                                                  0x91, 0x98, 0x8a, 0xf5];

        let decrypted_firmware_data: [u8; 100] = [0x4c, 0x6f, 0x72, 0x65, 0x6d, 0x20, 0x69, 0x70,
                                                  0x73, 0x75, 0x6d, 0x20, 0x64, 0x6f, 0x6c, 0x6f,
                                                  0x72, 0x20, 0x73, 0x69, 0x74, 0x20, 0x61, 0x6d,
                                                  0x65, 0x74, 0x2c, 0x20, 0x63, 0x6f, 0x6e, 0x73,
                                                  0x65, 0x63, 0x74, 0x65, 0x74, 0x75, 0x72, 0x20,
                                                  0x61, 0x64, 0x69, 0x70, 0x69, 0x73, 0x63, 0x69,
                                                  0x6e, 0x67, 0x20, 0x65, 0x6c, 0x69, 0x74, 0x2e,
                                                  0x20, 0x50, 0x72, 0x6f, 0x69, 0x6e, 0x20, 0x66,
                                                  0x61, 0x63, 0x69, 0x6c, 0x69, 0x73, 0x69, 0x73,
                                                  0x20, 0x72, 0x68, 0x6f, 0x6e, 0x63, 0x75, 0x73,
                                                  0x20, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x72, 0x61,
                                                  0x74, 0x20, 0x63, 0x72, 0x61, 0x73, 0x20, 0x61,
                                                  0x6d, 0x65, 0x74, 0x0a];

        let mut output_buffer = Vec::<u8>::with_capacity(super::EVIC_MINI_ROM_SIZE);
        let mut cursor = Cursor::new(encrypted_firmware_data.as_ref());
        let firmware = Firmware::decrypt(&mut cursor).unwrap();

        assert!(firmware.save(&mut output_buffer).is_ok());
        assert_eq!(&decrypted_firmware_data[..], &output_buffer[..]);
    }

    #[test]
    fn it_fails_when_firmware_size_exceeds_capacity() {
        use std::io::Cursor;

        let firmware_data = [0 as u8; super::EVIC_MINI_ROM_SIZE + 1];

        assert!(Firmware::decrypt(&mut Cursor::new(firmware_data.as_ref())).is_err());
    }
}
