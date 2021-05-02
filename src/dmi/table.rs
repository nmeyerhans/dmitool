// Copyright Noah Meyerhans <frodo@morgul.net>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation; version 2.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
// 02110-1301, USA.

use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;
use std::str;

use crate::dmi::err;

const ENTRYPOINT: &str = "/sys/firmware/dmi/tables/smbios_entry_point";
#[allow(dead_code)]
const TABLES: &str = "/sys/firmware/dmi/tables/DMI";

#[allow(dead_code)]
pub struct Table {
    major: u8,
    minor: u8,
    rev: u8,
    bits: Vec<u8>,
    strings: Vec<String>,
}

fn read_entrypoint() -> Result<Vec<u8>, io::Error> {
    fs::read(Path::new(ENTRYPOINT))
}

fn print_header_32(_: &Vec<u8>) -> Result<(), err::DMIParserError> {
    eprintln!("32-bit header support is implemented");
    Ok(())
}

#[allow(dead_code)]
fn print_header_64(header: &[u8]) -> Result<(), err::DMIParserError> {
    if header[6] != 0x18 {
        println!("Got unexpected header length");
        return Err(err::DMIParserError::HeaderDataError);
    }
    let maj = header[7];
    let min = header[8];
    let rev = header[9];
    println!("SMBIOS spec version: {}.{}.{}", maj, min, rev);
    if header[0xa] == 0x1 {
        println!("Using SMBIOS 3.0 entrypoint");
    } else {
        println!("Unknown entrypoint revision {}", header[0xa]);
        return Err(err::DMIParserError::HeaderDataError);
    }

    // Is there a more efficient way to do this?
    let mut bytes: [u8; 8] = [0; 8];
    for i in 0..8 {
        bytes[i] = header[0x10 + i];
    }
    let table_addr: u64 = u64::from_le_bytes(bytes);
    println!("Table is at location 0x{:x}", table_addr);
    let mut f = File::open(TABLES)?;
    let mut buf = [0; 2];
    f.read(&mut buf)?;
    println!(" Header bytes 1 and 2 are: {:02x} {:02x}", buf[0], buf[1]);
    if buf[0] != 0x00 {
        println!("Skipping table with ID {:02x}", buf[0]);
        let _pos: u64 = f.seek(SeekFrom::Start(buf[1].into()))?;
    }
    loop {
        let _strings: Vec<String> = Vec::new();
        let s = match read_null_terminated_string(&f) {
            Ok(s) => s,
            Err(_e) => break,
        };
        if s.len() == 0 {
            break;
        }
        println!("Read a string! {}", s);
    }
    Ok(())
}

fn read_null_terminated_string(fh: &File) -> Result<String, io::Error> {
    let mut r = String::new();
    for byte in fh.bytes() {
        let byte = match byte {
            Ok(byte) => byte,
            Err(e) => return Err(e),
        };
        if byte == 0x0 {
            return Ok(r);
        }
        r.push(byte.into());
    }
    Ok(r)
}

impl Table {
    pub fn read() -> Result<Table, err::DMIParserError> {
        let mut res = Table {
            major: 0,
            minor: 0,
            rev: 0,
            bits: Vec::new(),
            strings: Vec::new(),
        };
        let ep: Vec<u8> = match read_entrypoint() {
            Ok(data) => data,
            Err(e) => return Err(err::DMIParserError::IOError(e)),
        };
        if str::from_utf8(&ep[0..4]).unwrap() == "_SM_" {
            println!("Found a 32 bit header!");
            let r = print_header_32(&ep);
            match r {
                Ok(_r) => println!("end of 32 bit header"),
                Err(e) => return Err(e),
            };
        } else if str::from_utf8(&ep[0..5]).unwrap() == "_SM3_" {
            println!("Found a 64 bit header!");
            //print_header_64(&ep)?;
            res.read_header_64(&ep)?;
        }
        Ok(res)
    }

    fn read_header_64(&mut self, header: &[u8]) -> Result<(), err::DMIParserError> {
        if header[6] != 0x18 {
            println!("Got unexpected header length");
            return Err(err::DMIParserError::HeaderDataError);
        }
        self.major = header[7];
        self.minor = header[8];
        self.rev = header[9];

        println!(
            "SMBIOS spec version: {}.{}.{}",
            self.major, self.minor, self.rev
        );
        if header[0xa] == 0x1 {
            println!("Using SMBIOS 3.0 entrypoint");
        } else {
            println!("Unknown entrypoint revision {}", header[0xa]);
            return Err(err::DMIParserError::HeaderDataError);
        }
        Ok(())
    }

    pub fn version(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.rev)
    }
}