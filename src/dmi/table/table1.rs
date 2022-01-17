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

use crate::dmi::table::Table;
use std::fmt;

impl Table {
    fn fmt_manufacturer(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_str(f, 0x04, "System Manufacturer")
    }
    fn fmt_product_name(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_str(f, 0x05, "Product Name")
    }
    fn fmt_product_version(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_str(f, 0x06, "Product Version")
    }
    fn fmt_product_serial(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_str(f, 0x07, "Product Serial")
    }
    fn fmt_sku(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_str(f, 0x19, "Product SKU")
    }
    fn fmt_family(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_str(f, 0x1a, "Product Family")
    }
    fn fmt_uuid(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Section 7.2.1 of SMBIOS spec 3.5.0
        write!(
            f,
            "UUID: {:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}",
            self.data.bits[11],
            self.data.bits[10],
            self.data.bits[9],
            self.data.bits[8],
            self.data.bits[13],
            self.data.bits[12],
            self.data.bits[15],
            self.data.bits[14],
            self.data.bits[16],
            self.data.bits[17],
        )?;
        writeln!(
            f,
            "-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.data.bits[18],
            self.data.bits[19],
            self.data.bits[20],
            self.data.bits[21],
            self.data.bits[22],
            self.data.bits[23],
        )
    }

    pub fn fmt_table1(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Table 1 (System Information)")?;
        //let len: u8 = self.data.bits[1];
        let len: u8 = self.size();
        // SMBIOS 2.0 uses len 0x8
        // SMBIOS 2.1-2.3.4 use len 0x19
        // Newer versions (2.4+) use len 0x1b
        if len >= 8 {
            self.fmt_manufacturer(f)?;
            self.fmt_product_name(f)?;
            self.fmt_product_version(f)?;
            self.fmt_product_serial(f)?;
        }

        if len >= 0x1b {
            self.fmt_sku(f)?;
            self.fmt_family(f)?;
        }

        if len > 8 {
            // Wake-up type is only defined for SMBIOS 2.1+, determined by the structure length
            let byte_values = [
                (0, "Reserved"),
                (1, "Other"),
                (2, "Unknown"),
                (3, "APM Timer"),
                (4, "Modem ring"),
                (5, "LAN Remote"),
                (6, "Power switch"),
                (7, "PCI PME#"),
                (8, "AC Power Restored"),
            ];
            let idx: usize = self.data.bits[0x18].into();
            writeln!(f, "Wake reason: {}", byte_values[idx].1)?;
        }

        if len >= 0x19 {
            self.fmt_uuid(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::dmi::table::Data;
    use crate::dmi::table::Table;
    use crate::dmi::table::TableId;
    #[test]
    // table with no meaningful data at all. sign of a buggy firmware
    fn test_empty_table() {
        let d = Data {
            location: 0,
            string_location: 0,
            next_loc: 0,
            bits: [].to_vec(),
            strings: [].to_vec(),
        };
        let table = Table {
            id: TableId::System,
            data: d,
        };
        let r = format!("{}", table);
        println!("{}", r);
        // Product name is in all versions:
        assert!(r.contains("Table 1 (System Information)"));
        assert!(!r.contains("Product Name"));
        // Product family is v2.4 extension
        assert!(!r.contains("Product Family"));
    }
    // table with data, but all zero. more buggy firmware
    #[test]
    fn test_zero_table() {
        let d = Data {
            location: 0,
            string_location: 0,
            next_loc: 0,
            bits: [0, 0, 0, 0, 0, 0, 0, 0].to_vec(),
            strings: [
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
            ]
            .to_vec(),
        };
        let table = Table {
            id: TableId::System,
            data: d,
        };
        let r = format!("{}", table);
        println!("{}", r);
        // Product name is in all versions:
        assert!(r.contains("Table 1 (System Information)"));
        assert!(!r.contains("Product Name"));
        // Product family is v2.4 extension
        assert!(!r.contains("Product Family"));
    }

    #[test]
    fn test_manufacturer() {
        let d = Data {
            location: 0,
            string_location: 0,
            next_loc: 0,
            // bits[4] points to the manufacturer string
            bits: [1, 8, 0, 0, 2, 0, 0, 0].to_vec(),
            strings: [String::from(""), String::from("ACME Widgets, Inc.")].to_vec(),
        };
        let table = Table {
            id: TableId::System,
            data: d,
        };
        let r = format!("{}", table);
        println!("{}", r);
        assert!(r.contains("System Manufacturer: ACME Widgets, Inc."));
    }

    #[test]
    fn test_string_index_out_of_range() {
        let d = Data {
            location: 0,
            string_location: 0,
            next_loc: 0,
            // bits[5] should point to the product name string...
            bits: [0, 8, 0, 0, 0, 100, 0, 0].to_vec(),
            // but strings[100] is out of bounds:
            strings: [String::from("")].to_vec(),
        };
        let table = Table {
            id: TableId::System,
            data: d,
        };
        let r = format!("{}", table);
        println!("{}", r);
        assert!(r.contains("Product Name: Unknown. Buggy firmware."));
    }

    #[test]
    fn test_v2_table() {
        let d = Data {
            location: 0,
            string_location: 0,
            next_loc: 0,
            bits: [1, 8, 0, 0, 1, 2, 3, 4].to_vec(),
            strings: [
                String::from("test manufacturer"),
                String::from("test name"),
                String::from("test version"),
                String::from("test serial"),
            ]
            .to_vec(),
        };
        let table = Table {
            id: TableId::System,
            data: d,
        };
        let r = format!("{}", table);
        println!("{}", r);
        // Product manufacturer, name, version, and serial are present
        // in all v2+ versions:
        assert!(r.contains("Product Name: test name"));
        // Product family is v2.4 extension
        assert!(!r.contains("Product Family"));
    }
}
