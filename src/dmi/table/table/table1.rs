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

pub mod table2 {

    use crate::table::Table;
    use std::fmt;

    impl Table {
        fn fmt_manufacturer(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.fmt_str(f, 4, "System Manufacturer")
        }
        fn fmt_product_name(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.fmt_str(f, 5, "Product Name")
        }
        fn fmt_product_version(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.fmt_str(f, 6, "Product Version")
        }
        fn fmt_product_serial(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.fmt_str(f, 7, "Product Serial")
        }
        fn fmt_sku(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.fmt_str(f, 0x19, "Product SKU")
        }
        fn fmt_family(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.fmt_str(f, 0x1a, "Product Family")
        }
        fn fmt_uuid(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // TODO: decode the system UUID
            write!(
                f,
                "UUID: Seems to be present but decoding is not yet supported\n"
            )
        }

        pub fn fmt_table1(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let len: u8 = self.data.bits[1];
            // SMBIOS 2.0 uses len 0x8
            // SMBIOS 2.1-2.3.4 use len 0x19
            // Newer versions (2.4+) use len 0x1b
            write!(f, "Table 1 (System Information)\n")?;
            self.fmt_manufacturer(f)?;
            self.fmt_product_name(f)?;
            self.fmt_product_version(f)?;
            self.fmt_product_serial(f)?;
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
                write!(f, "Wake reason: {}\n", byte_values[idx].1)?;
            }

            if len >= 0x19 {
                self.fmt_uuid(f)?;
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::table::Data;
    use crate::table::Table;
    use crate::table::TableId;
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
        assert!(r.contains("Product Name: Unspecified"));
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
            bits: [0, 0, 0, 0, 1, 0, 0, 0].to_vec(),
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
            // bits[5] should point to the product version string...
            bits: [0, 0, 0, 0, 0, 100, 0, 0].to_vec(),
            // but strings[100] is out of bounds:
            strings: [].to_vec(),
        };
        let table = Table {
            id: TableId::System,
            data: d,
        };
        let r = format!("{}", table);
        println!("{}", r);
        assert!(!r.contains("ACME Widgets, Inc."));
    }
}
