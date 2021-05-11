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

#[allow(dead_code)]
pub struct Entrypoint {
    major: u8,
    minor: u8,
    rev: u8,
    length: u8,
    location: u64,
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
            debug!("Found a 32 bit header!");
            Entrypoint::from_header_32(&ep)
        } else if str::from_utf8(&ep[0..5]).unwrap() == "_SM3_" {
            debug!("Found a 64 bit header!");
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
            error!("Got unexpected header length");
            return Err(err::DMIParserError::HeaderDataError);
        }

        info!(
            "SMBIOS spec version: {}.{}.{}",
            header[7], header[8], header[9]
        );
        if header[0xa] == 0x1 {
            info!("Using SMBIOS 3.0 entrypoint");
        } else {
            error!("Unknown entrypoint revision {}", header[0xa]);
            return Err(err::DMIParserError::HeaderDataError);
        }
        // Is there a more efficient way to do this?
        let mut bytes: [u8; 8] = [0; 8];
        for i in 0..8 {
            bytes[i] = header[0x10 + i];
        }
        let table_addr: u64 = u64::from_le_bytes(bytes);
        info!("Table is at location 0x{:x}", table_addr);

        Ok(Entrypoint {
            major: header[7],
            minor: header[8],
            rev: header[9],
            length: header[6],
            location: table_addr,
        })
    }

    pub fn version(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.rev)
    }
}
