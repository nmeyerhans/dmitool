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

#[derive(Debug)]
#[allow(dead_code)]
pub struct Table {
    pub location: u64,
    pub string_location: u64,
    pub next_loc: u64,
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
        Table::read_at(0)
    }

    pub fn read_at(loc: u64) -> Result<Table, err::DMIParserError> {
        let mut f = File::open(TABLES)?;
        f.seek(SeekFrom::Start(loc))?;
        let mut buf = [0; 256];
        // read the header, which gives us the table ID and size
        let res = f.read(&mut buf[0..4])?;

        let offset: usize = 4;
        let end: usize = (buf[1]).into();
        let res = f.read(&mut buf[offset..end])?;

        let string_location = f.stream_position()?;
        let mut strings: Vec<String> = Vec::new();
        loop {
            let s = match read_null_terminated_string(&f) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("While reading strings: {}", e);
                    break;
                }
            };
            if s.is_empty() {
                if strings.len() == 0 {
                    // special case: this table structure has no strings
                    f.seek(SeekFrom::Current(1))?;
                }
                break;
            }
            strings.push(s);
        }
        Ok(Table {
            location: loc,
            string_location: string_location,
            next_loc: f.stream_position()?,
            bits: buf.to_vec(),
            strings: strings,
        })
    }

    pub fn id(&self) -> u8 {
        self.bits[0]
    }

    pub fn size(&self) -> u8 {
        self.bits[1]
    }

    pub fn handle(&self) -> u16 {
        // FIXME: What's the right way to do this?
        let foo: u16 = self.bits[2].into();
        let l: u16 = self.bits[3].into();
        (l << 8) | foo
    }
}
