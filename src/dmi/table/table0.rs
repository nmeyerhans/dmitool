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

// use crate::dmi::table::decode_byte;
use crate::dmi::table::decode_byte;
use crate::dmi::table::Table;
use std::fmt;

impl Table {
    fn fmt_vendor(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_str(f, 4, "BIOS Vendor")
    }

    fn fmt_version(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_str(f, 5, "BIOS Version")
    }

    fn fmt_date(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_str(f, 8, "BIOS Release Date")
    }

    fn decode_bios_extension_byte1(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b: u8 = self.data.bits[18];
        let bit_strings = [
            (1, "ACPI is supported"),
            (1 << 1, "USB Legacy is supported"),
            (1 << 2, "AGP is supported"),
            (1 << 3, "I2O boot is supported"),
            (1 << 4, "LS-120 SuperDisk boot is supported"),
            (1 << 5, "ATAPI ZIP drive boot is supported"),
            (1 << 6, "1394 boot is supported"),
            (1 << 7, "Smart battery is supported"),
        ];
        println!("BIOS Characteristics Extension byte 1:");
        for bit in bit_strings.iter() {
            if (b & bit.0) != 0 {
                writeln!(f, "  + {}", bit.1)?;
            }
        }
        Ok(())
    }

    fn decode_bios_extension_byte2(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b: u8 = self.data.bits[19];
        let bit_strings = [
            (1, "BIOS Boot Specification is supported"),
            (1 << 1, "F-Key initiated network boot is supported"),
            (1 << 2, "Enable targeted content distribution"),
            (1 << 3, "UEFI Specification is supported"),
            (1 << 4, "SMBIOS table describes a virtual machine"),
            /* Remaining bits are reserved for future use */
        ];
        println!("BIOS Characteristics Extension byte 2:");
        for bit in bit_strings.iter() {
            if (b & bit.0) != 0 {
                writeln!(f, "  + {}", bit.1)?;
            }
        }
        Ok(())
    }

    pub fn fmt_table0(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len: u8 = self.data.bits[1];
        if len < 0x12 {
            write!(f, "Invalid BIOS characteristics table length {}", len)?;
            return Err(std::fmt::Error);
        }
        let pos = 0xa;
        if self.data.bits[pos] & (1 << 3) != 0 {
            // Does "BIOS Characteristics are not supported" really mean we should skip this table?
            write!(f, "BIOS Characteristics not supported on this system")?;
            return Err(std::fmt::Error);
        }
        writeln!(f, "BIOS Characteristics")?;
        writeln!(f, "Table handle is {}", self.handle())?;

        let bit_strings = [
            (1 << 4, "ISA is supported"),
            (1 << 5, "MCA is supported"),
            (1 << 6, "EISA is supported"),
            (1 << 7, "PCI is supported"),
        ];
        decode_byte(f, self.data.bits[pos], &bit_strings)?;
        let pos = pos + 1;
        let bit_strings = [
            (1, "PCMCI is supported"),
            (1 << 1, "PnP is supported"),
            (1 << 2, "APM is supported"),
            (1 << 3, "BIOS upgrades are supported"),
            (1 << 4, "BIOS shadowing is allowed"),
            (1 << 5, "VL-VESA is supported"),
            (1 << 6, "ESCD support is available"),
            (1 << 7, "Boot from CD is supported"),
        ];
        decode_byte(f, self.data.bits[pos], &bit_strings)?;
        let pos = pos + 1;
        let bit_strings = [
            (1, "Selectable boot is supported"),
            (1 << 1, "BIOS ROM is socketed"),
            (1 << 2, "Boot from PCMCIA (PC Card) is supported"),
            (1 << 3, "EDD Specification is supported"),
            (1 << 4, "Int 13h: NEC 9800 1.2 MB floppy is supported"),
            (1 << 5, "Int 13h: Toshiba 1.2 MB Floppy is supported"),
            (1 << 6, "Int 13h: 5.25” 360 KB floppy is supportd"),
            (1 << 7, "Int 13h: 5.25” 1.2 MB floppy is supported"),
        ];
        decode_byte(f, self.data.bits[pos], &bit_strings)?;
        let pos = pos + 1;
        let bit_strings = [
            (1, "Int 13h: 3.5” / 720 KB floppy services are supported"),
            (
                1 << 1,
                "Int 13h: 3.5” / 2.88 MB floppy services are supported",
            ),
            (1 << 2, "Int 5h: print screen Service is supported"),
            (1 << 3, "Int 9h: 8042 keyboard services are supported"),
            (1 << 4, "Int 14h: serial services are supported"),
            (1 << 5, "Int 17h: printer services are supported"),
            (1 << 6, "Int 10h: CGA/Mono Video Services are supported"),
            (1 << 7, "NEC PC-98"),
        ];
        decode_byte(f, self.data.bits[pos], &bit_strings)?;

        if len > 19 {
            self.decode_bios_extension_byte1(f)?;
        }
        if len > 20 {
            self.decode_bios_extension_byte2(f)?;
        }
        self.fmt_vendor(f)?;
        self.fmt_version(f)?;
        self.fmt_date(f)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::dmi::table::Data;
    use crate::dmi::table::Table;
    use crate::dmi::table::TableId;
    #[test]
    fn test_decode_table0_v20() {
        let d = Data {
            location: 0,
            string_location: 0,
            next_loc: 0,
            bits: [
                0,    //type
                0x12, // length is 0x12 + entension bytes (zero here)
                0x0,  // handle (2 bytes)
                0x1,
                1,    // BIOS vendor name string
                2,    // BIOS version string
                0x80, // BIOS starting address segment (2 bytes)
                0x80,
                3,      // BIOS release date string
                2,      // BIOS size (n + 1 64 kB chunks)
                128,    // BIOS characteristics (8 bytes)
                128,    // CDROM boot is supported
                1,      // selectable boot is supported
                1 << 4, // serial support
                0,
                0,
                0,
                0,
            ]
            .to_vec(),
            strings: [
                String::from("ACME Widgets, Inc."),
                String::from("1.0a+3"),
                String::from("5/8/1977"),
            ]
            .to_vec(),
        };
        let table = Table {
            id: TableId::Bios,
            data: d,
        };
        let r = format!("{}", table);
        println!("{}", r);
        assert!(r.contains("BIOS Characteristics"));
        assert!(r.contains("Table handle is 256"));
        assert!(r.contains("BIOS Vendor: ACME Widgets, Inc."));
        assert!(r.contains("BIOS Version: 1.0a+3"));
        assert!(r.contains("BIOS Release Date: 5/8/1977"));
        assert!(r.contains("PCI is supported"));
        assert!(r.contains("Boot from CD is supported"));
        assert!(r.contains("Selectable boot is supported"));
        assert!(r.contains("Int 14h: serial services are supported"));
    }
}
