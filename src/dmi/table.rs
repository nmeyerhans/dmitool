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
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;

mod table0;

const TABLES: &str = "/sys/firmware/dmi/tables/DMI";

#[derive(Debug)]
#[allow(dead_code)]
struct Data {
    pub location: u64,
    pub string_location: u64,
    pub next_loc: u64,
    pub bits: Vec<u8>,
    pub strings: Vec<String>,
}

#[allow(dead_code)]
enum TableId {
    BIOS,
    System,
    Baseboard,
    Chassis,
    Other,
}

#[allow(dead_code)]
pub struct Table {
    id: TableId,
    data: Data,
}

#[allow(dead_code)]
fn print_header_64(_header: &[u8]) -> Result<(), err::DMIParserError> {
    let mut f = File::open(TABLES)?;
    let mut buf = [0; 2];
    f.read(&mut buf)?;
    debug!(" Header bytes 1 and 2 are: {:02x} {:02x}", buf[0], buf[1]);
    if buf[0] != 0x00 {
        debug!("Skipping table with ID {:02x}", buf[0]);
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
        debug!("Read a string! {}", s);
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

#[allow(dead_code)]
impl Table {
    pub fn read() -> Result<Table, err::DMIParserError> {
        Table::read_at(0)
    }

    pub fn read_at(loc: u64) -> Result<Table, err::DMIParserError> {
        let f = File::open(TABLES)?;
        Table::read_fh_at(f, loc)
    }

    pub fn read_fh_at(mut f: File, location: u64) -> Result<Table, err::DMIParserError> {
        f.seek(SeekFrom::Start(location))?;
        let mut buf = [0; 256];
        // read the header, which gives us the table ID and size
        let _res = f.read(&mut buf[0..4])?;
        if buf[0] == 0 {
            debug!(
                "Read header bytes: {:02x} {:02x} {:02x} {:02x}",
                buf[0], buf[1], buf[2], buf[3]
            );
        }
        let offset: usize = 4;
        let end: usize = (buf[1]).into();
        let _res = f.read(&mut buf[offset..end])?;
        if buf[0] == 0 {
            for byte in 0..1 {
                debug!(
                    "Read header bytes: {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}",
                    buf[byte + 0],
                    buf[byte + 1],
                    buf[byte + 2],
                    buf[byte + 3],
                    buf[byte + 4],
                    buf[byte + 5],
                    buf[byte + 6],
                    buf[byte + 7],
                );
            }
        }

        let string_location = f.stream_position()?;
        let mut strings: Vec<String> = Vec::new();
        loop {
            let s = match read_null_terminated_string(&f) {
                Ok(s) => s,
                Err(e) => {
                    error!("While reading strings: {}", e);
                    break;
                }
            };
            debug!("Read string {}", s);
            if s.is_empty() {
                if strings.len() == 0 {
                    // special case: this table structure has no strings
                    f.seek(SeekFrom::Current(1))?;
                }
                break;
            }
            strings.push(s);
        }
        let res = Data {
            location: location,
            string_location: string_location,
            next_loc: f.stream_position()?,
            bits: buf.to_vec(),
            strings: strings,
        };
        match res.bits[0] {
            0 => Ok(Table {
                id: TableId::BIOS,
                data: res,
            }),
            _ => Ok(Table {
                id: TableId::Other,
                data: res,
            }),
        }
    }

    pub fn id(&self) -> u8 {
        self.data.bits[0]
    }

    #[allow(dead_code)]
    pub fn size(&self) -> u8 {
        self.data.bits[1]
    }

    pub fn handle(&self) -> u16 {
        // FIXME: What's the right way to do this?
        let foo: u16 = self.data.bits[2].into();
        let l: u16 = self.data.bits[3].into();
        (l << 8) | foo
    }

    pub fn strings(&self) -> &Vec<String> {
        &self.data.strings
    }

    pub fn bits(&self) -> &Vec<u8> {
        &self.data.bits
    }

    pub fn location(&self) -> u64 {
        self.data.location
    }

    pub fn next_loc(&self) -> u64 {
        self.data.next_loc
    }
}

fn decode_byte(f: &mut fmt::Formatter<'_>, b: u8, bit_strings: &[(u8, &str)]) -> fmt::Result {
    for bit in bit_strings.iter() {
        if (b & bit.0) != 0 {
            write!(f, "  + {}\n", bit.1)?;
        }
    }
    Ok(())
}

fn fmt_unknown_table(f: &mut fmt::Formatter<'_>, data: &Vec<u8>) -> fmt::Result {
    write!(f, "Unhandled table {}\n", data[0])
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.data.bits[0] {
            0 => self.fmt(f),
            _ => fmt_unknown_table(f, &self.data.bits),
        }
    }
}
