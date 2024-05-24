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

use crate::dmi::table::decode_byte;
use crate::dmi::table::Table;
use std::fmt;

impl Table {
    pub fn fmt_baseboard_manufacturer(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_str(f, 4, "Manufacturer")
    }
    pub fn fmt_baseboard_product(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_str(f, 5, "Product")
    }
    pub fn fmt_baseboard_version(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_str(f, 6, "Version")
    }
    pub fn fmt_baseboard_serial(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_str(f, 7, "Serial")
    }
    pub fn fmt_baseboard_asset_tag(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_str(f, 8, "Asset tag")
    }
    pub fn fmt_board_location(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	self.fmt_str(f, 0xa, "Location in chassis")
    }
    pub fn fmt_feature_flags(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let byte: u8 = self.data.bits[0x9];
        let bit_strings = [
            (1, "Board is a hosting board"),
            (1 << 1, "At least one daughterboard is required"),
            (1 << 2, "Board is removable"),
            (1 << 3, "Board is replaceable"),
            (1 << 4, "Board is hot swappable"),
        ];
        writeln!(f, "Baseboard features:")?;
        decode_byte(f, byte, &bit_strings)
    }

    pub fn fmt_board_type(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t: &str = match self.data.bits[0xd] {
            0x1 => "Unknown",
            0x2 => "Other",
            0x3 => "Server Blade",
            0x4 => "Connectivity Switch",
            0x5 => "System Management Module",
            0x6 => "Processor Module",
            0x7 => "I/O Module",
            0x8 => "Memory Module",
            0x9 => "Daughter board",
            0xa => "Motherboard (includes processor, memory, and I/O)",
            0xb => "Processor/Memory Module",
            0xc => "Processor/IO Module",
            0xd => "Interconnect board",
            _ => "Unrecognized value. Probably a bug.",
        };
        writeln!(f, "Board type: {}", t)
    }
    pub fn fmt_table2(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len = self.data.bits[1];
        writeln!(f, "Table 2 (Baseboard Information)")?;
        self.fmt_baseboard_manufacturer(f)?;
        self.fmt_baseboard_product(f)?;
        self.fmt_baseboard_version(f)?;
        self.fmt_baseboard_serial(f)?;
        self.fmt_baseboard_asset_tag(f)?;
	self.fmt_board_location(f)?;
        if len > 0x8 {
            self.fmt_feature_flags(f)?;
        }

        if len > 0xd {
            self.fmt_board_type(f)?;
        }

        // TODO: format additional strings, bitfields, etc
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::dmi::table::Data;
    use crate::dmi::table::Table;
    use crate::dmi::table::TableId;
    #[test]
    fn test_table2() {
        let d = Data {
            location: 0,
            string_location: 0,
            next_loc: 0,
            bits: [
                0x2, // type
                0xa, // length
                0x0, // handle, 2 bytes
                0x1, // handle (cont)
		0x1, // Manufacturer string
                0x2, // Product string
                0x3, // Version string
                0x4, // Serial Number string
                0x5, // Asset Tag string
		0x0, // Feature flags
		0x6, // Location in chassis
            ]
            .to_vec(),
            strings: [
                String::from("ACME Widgets, Inc."),
                String::from("Illudium Q-36 Explosive Space Modulator"),
                String::from("0.1.2"),
                String::from("ABCDabcd"),
                String::from("My Asset Tag"),
		String::from("Some location"),
            ]
            .to_vec(),
        };
        let table = Table {
            id: TableId::Baseboard,
            data: d,
        };
        let r = format!("{}", table);
        println!("{}", r);
        assert!(r.contains("Manufacturer: ACME Widgets, Inc."));
        assert!(r.contains("Product: Illudium Q-36 Explosive Space Modulator"));
        assert!(r.contains("Version: 0.1.2"));
        assert!(r.contains("Serial: ABCDabcd"));
        assert!(r.contains("Asset tag: My Asset Tag"));
	assert!(r.contains("Location in chassis: Some location"));
    }

    #[test]
    fn test_table2_large() {
        let d = Data {
            location: 0,
            string_location: 0,
            next_loc: 0,
            bits: [
                0x2,  // type
                0xe,  // length
                0x0,  // handle, byte 1
                0x1,  // handle, byte 1
                0x1,  // Manufacturer string
                0x2,  // Product string
                0x3,  // Version string
                0x4,  // Serial Number string
                0x5,  // Asset Tag string
                0x1f, // Features
                0x6,  // Location in chassis string,
                0,    // Chassis handle byte 1
                0,    // Chassis handle byte 2
                0xa,  // Board type (motherboard)
            ]
            .to_vec(),
            strings: [
                String::from("ACME Widgets, Inc."),
                String::from("Illudium Q-36 Explosive Space Modulator"),
                String::from("0.1.2"),
                String::from("ABCDabcd"),
                String::from("My Asset Tag"),
		String::from("Nubus slot 7-11"),
            ]
            .to_vec(),
        };
        let table = Table {
            id: TableId::Baseboard,
            data: d,
        };
        let r = format!("{}", table);
        println!("{}", r);
        assert!(r.contains("Manufacturer: ACME Widgets, Inc."));
        assert!(r.contains("Product: Illudium Q-36 Explosive Space Modulator"));
        assert!(r.contains("Version: 0.1.2"));
        assert!(r.contains("Serial: ABCDabcd"));
        assert!(r.contains("Asset tag: My Asset Tag"));
        assert!(r.contains("  + Board is a hosting board"));
        assert!(r.contains("  + At least one daughterboard is required"));
        assert!(r.contains("  + Board is removable"));
        assert!(r.contains("  + Board is replaceable"));
        assert!(r.contains("  + Board is hot swappable"));
        assert!(r.contains("Board type: Motherboard (includes processor, memory, and I/O)"));
	assert!(r.contains("Location in chassis: Nubus slot 7-11"));
    }

    #[test]
    fn test_table2_short_length() {
        let d = Data {
            location: 0,
            string_location: 0,
            next_loc: 0,
            bits: [
                0x2, // type
                0x8, // length
                0x0, 0x1, // handle, 2 bytes
                0x1, // Manufacturer string
                0x2, // Product string
                0x3, // Version string
                0x4, // Serial Number string
                0x5, // Asset Tag string
		0x0, // Feature flags
		0x6, // Location in chassis
            ]
            .to_vec(),
            strings: [].to_vec(),
        };
        let table = Table {
            id: TableId::Baseboard,
            data: d,
        };
        let r = format!("{}", table);
        println!("{}", r);
        assert!(r.contains("Manufacturer: String index out of range. Buggy firmware?"));
        assert!(r.contains("Product: String index out of range. Buggy firmware?"));
        assert!(r.contains("Version: String index out of range. Buggy firmware?"));
        assert!(r.contains("Serial: String index out of range. Buggy firmware?"));
        assert!(r.contains("Asset tag: String index out of range. Buggy firmware?"));
	assert!(r.contains("Location in chassis: String index out of range. Buggy firmware?"));
    }
}
