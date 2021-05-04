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
use std::fs;
use std::io;
use std::path::Path;
use std::str;

const ENTRYPOINT: &str = "/sys/firmware/dmi/tables/smbios_entry_point";
pub const TABLES: &str = "/sys/firmware/dmi/tables/DMI";

pub struct Entrypoint {
    major: u8,
    minor: u8,
    rev: u8,
}

fn read_entrypoint() -> Result<Vec<u8>, io::Error> {
    fs::read(Path::new(ENTRYPOINT))
}

impl Entrypoint {
    pub fn read() -> Result<Entrypoint, err::DMIParserError> {
        let ep: Vec<u8> = match read_entrypoint() {
            Ok(data) => data,
            Err(e) => return Err(err::DMIParserError::IOError(e)),
        };
        if str::from_utf8(&ep[0..4]).unwrap() == "_SM_" {
            eprintln!("Found a 32 bit header!");
            Entrypoint::from_header_32(&ep)
        } else if str::from_utf8(&ep[0..5]).unwrap() == "_SM3_" {
            println!("Found a 64 bit header!");
            Entrypoint::from_header_64(&ep)
        } else {
            Err(err::DMIParserError::HeaderDataError)
        }
    }

    fn from_header_32(_header: &[u8]) -> Result<Entrypoint, err::DMIParserError> {
        Err(err::DMIParserError::NotImplemented)
    }

    fn from_header_64(header: &[u8]) -> Result<Entrypoint, err::DMIParserError> {
        if header[6] != 0x18 {
            println!("Got unexpected header length");
            return Err(err::DMIParserError::HeaderDataError);
        }

        println!(
            "SMBIOS spec version: {}.{}.{}",
            header[7], header[8], header[9]
        );
        if header[0xa] == 0x1 {
            println!("Using SMBIOS 3.0 entrypoint");
        } else {
            println!("Unknown entrypoint revision {}", header[0xa]);
            return Err(err::DMIParserError::HeaderDataError);
        }
        Ok(Entrypoint {
            major: header[7],
            minor: header[8],
            rev: header[9],
        })
    }

    pub fn version(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.rev)
    }
}
