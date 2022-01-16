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

pub mod table {

    use crate::dmi::err;
    use std::fmt;
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    use std::io::SeekFrom;

    mod table0;
    mod table1;
    mod table2;

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

    enum TableId {
        Bios,
        System,
        Baseboard,
        Chassis,
        Other,
    }

    pub struct Table {
        id: TableId,
        data: Data,
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
            let f = File::open(TABLES)?;
            Table::read_fh_at(f, loc)
        }

        pub fn read_fh_at(mut f: File, location: u64) -> Result<Table, err::DMIParserError> {
            f.seek(SeekFrom::Start(location))?;
            let mut buf = [0; 256];
            // read the header, which gives us the table ID and size
            let _res = f.read(&mut buf[0..4])?;
            debug!(
                "Read header bytes: {:02x} {:02x} {:02x} {:02x}",
                buf[0], buf[1], buf[2], buf[3]
            );
            f.seek(SeekFrom::Start(location))?;
            let end: usize = (buf[1]).into();
            let _res = f.read(&mut buf[0..end])?;
            debug!(
                "Read header bytes: {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}",
                buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
            );

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
                    if strings.is_empty() {
                        // special case: this table structure has no strings
                        f.seek(SeekFrom::Current(1))?;
                    }
                    break;
                }
                strings.push(s);
            }
            let res = Data {
                location,
                string_location,
                next_loc: f.stream_position()?,
                bits: buf.to_vec(),
                strings,
            };
            match res.bits[0] {
                0 => Ok(Table {
                    id: TableId::Bios,
                    data: res,
                }),
                1 => Ok(Table {
                    id: TableId::System,
                    data: res,
                }),
                2 => Ok(Table {
                    id: TableId::Baseboard,
                    data: res,
                }),
                3 => Ok(Table {
                    id: TableId::Chassis,
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

        pub fn size(&self) -> u8 {
            match self.data.bits.len() {
                0 => 0,
                _ => self.data.bits[1],
            }
        }

        pub fn handle(&self) -> u16 {
            let low: u16 = self.data.bits[2].into();
            let high: u16 = self.data.bits[3].into();
            (high << 8) | low
        }

        pub fn strings(&self) -> &Vec<String> {
            &self.data.strings
        }

        pub fn location(&self) -> u64 {
            self.data.location
        }

        pub fn next_loc(&self) -> u64 {
            self.data.next_loc
        }

        pub fn fmt_str(&self, f: &mut fmt::Formatter<'_>, index: u8, label: &str) -> fmt::Result {
            let mut val: &str = "Unknown. Buggy firmware.";

            let idx: usize = (self.data.bits[usize::from(index)]).into();
            if idx > 0 && self.data.strings.len() >= idx {
                if !self.data.strings[idx - 1].is_empty() {
                    val = &self.data.strings[idx - 1];
                } else {
                    val = "Unspecified";
                }
            }
            writeln!(f, "{}: {}", label, val)
        }
    }

    fn decode_byte(f: &mut fmt::Formatter<'_>, b: u8, bit_strings: &[(u8, &str)]) -> fmt::Result {
        for bit in bit_strings.iter() {
            if (b & bit.0) != 0 {
                writeln!(f, "  + {}", bit.1)?;
            }
        }
        Ok(())
    }

    fn fmt_unknown_table(f: &mut fmt::Formatter<'_>, data: &[u8]) -> fmt::Result {
        writeln!(f, "Unhandled table {}", data[0])
    }

    impl fmt::Display for Table {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self.id {
                TableId::Bios => self.fmt_table0(f),
                TableId::System => self.fmt_table1(f),
                TableId::Baseboard => self.fmt_table2(f),
                _ => fmt_unknown_table(f, &self.data.bits),
            }
        }
    }
}
