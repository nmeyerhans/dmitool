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

use crate::dmi::err;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;

const TABLES: &str = "/sys/firmware/dmi/tables/DMI";

#[allow(dead_code)]
pub struct Table {
    pub bits: Vec<u8>,
    pub strings: Vec<String>,
}

#[allow(dead_code)]
fn print_header_64(_header: &[u8]) -> Result<(), err::DMIParserError> {
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
        let mut f = File::open(TABLES)?;
        let mut buf = [0; 256];
        // read the header, which gives us the table ID and size
        f.read_exact(&mut buf[0..3])?;
        println!(" Header bytes 1 and 2 are: {:02x} {:02x}", buf[0], buf[1]);

        let offset: usize = 4;
        let end: usize = buf[1].into();
        f.read(&mut buf[offset..end])?;

        let mut strings: Vec<String> = Vec::new();
        loop {
            let s = match read_null_terminated_string(&f) {
                Ok(s) => s,
                Err(_e) => break,
            };
            if s.len() == 0 {
                break;
            }
            println!("Read a string! {}", s);
            strings.push(s);
        }
        Ok(Table {
            bits: buf.to_vec(),
            strings: strings,
        })
    }

    pub fn id(&self) -> u8 {
        self.bits[0]
    }

    pub fn handle(&self) -> u16 {
        // FIXME: What's the right way to do this?
        let foo: u16 = self.bits[2].into();
        let l: u16 = self.bits[3].into();
        foo << 8 | l
    }
}
