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

#[derive(Debug)]
enum TableLocation {
    Loc32(u32),
    Loc64(u64),
}

// 32 bit headers track the size of the complete structure table as a
// 16-bit "Structure Table Length" value, while 64 bit headers use a
// 32-bit Structure Table Maximum Size value.
#[derive(Debug)]
enum TableSize {
    MaxSize(u32),
    Length(u16),
}

#[derive(Debug)]
pub struct Entrypoint {
    major: u8,
    minor: u8,
    rev: u8,
    length: u8,
    location: TableLocation,
    table_size: TableSize,
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

    fn from_header_32(header: &[u8]) -> Result<Entrypoint, err::DMIParserError> {
        if header[5] != 0x1f {
            error!("Got unexpected header length");
            return Err(err::DMIParserError::HeaderDataError);
        }
        info!(
            "SMBIOS spec version: {}.{}.{}",
            header[6], header[7], header[8]
        );
        if header[0xa] == 0 {
            info!("Using SMBIOS 2.1 entrypoint");
        } else {
            error!("Unsupported entrypoint revision {}", header[0xa]);
            return Err(err::DMIParserError::HeaderDataError);
        }

        let mut bytes: [u8; 4] = [0; 4];
        for i in 0..4 {
            bytes[i] = header[0x18 + i];
        }
        let table_addr: u32 = u32::from_le_bytes(bytes);

        let mut bytes: [u8; 2] = [0; 2];
        for i in 0..2 {
            bytes[i] = header[0x16 + i];
        }
        let table_len = u16::from_le_bytes(bytes);

        debug!(
            "Structure table is at 0x{:04x}, lenght 0x{:02x}",
            table_addr, table_len
        );

        let mut bytes: [u8; 2] = [0; 2];
        for i in 0..2 {
            bytes[i] = header[0x16 + i];
        }
        let structure_max_size = u16::from_le_bytes(bytes);

        let ep = Entrypoint {
            major: header[6],
            minor: header[7],
            rev: header[8],
            length: header[5],
            location: TableLocation::Loc32(table_addr),
            table_size: TableSize::Length(structure_max_size.into()),
        };
	debug!("Read 32 bit entrypoint {:?}", ep);
	Ok(ep)
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

        let mut bytes: [u8; 4] = [0; 4];
        for i in 0..4 {
            bytes[i] = header[0xc + i];
        }
        let structure_max_size: u32 = u32::from_le_bytes(bytes);
        debug!("Table structrure max size is 0x{:04x}", structure_max_size);

        let ep = Entrypoint {
            major: header[7],
            minor: header[8],
            rev: header[9],
            length: header[6],
            location: TableLocation::Loc64(table_addr),
            table_size: TableSize::MaxSize(structure_max_size),
        };
	debug!("Read 64 bit entrypoint {:?}", ep);
	Ok(ep)
    }

    pub fn version(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.rev)
    }
    pub fn table_size(&self) -> u32 {
	match self.table_size {
            TableSize::MaxSize(v) => v,
	    TableSize::Length(v) => v.into(),
	}
    }
}
