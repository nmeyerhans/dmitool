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
    pub fn fmt_chassis_manufacturer(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_str(f, 4, "Manufacturer")
    }

    fn decode_chassis_type_byte(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.data.bits[5] >> 7 == 1 {
            writeln!(f, "Chassis lock is present")?;
        } else {
            writeln!(f, "Chassis lock not known to be present")?;
        }

        debug!("chassis type is {}", self.data.bits[5] & 127);

        let t: &str = match self.data.bits[5] & 127 {
            0x1 => "Other",
            0x2 => "Unknown",
            0x3 => "Desktop",
            0x4 => "Low Profile Desktop",
            0x5 => "Pizza Box",
            0x6 => "Mini Tower",
            0x7 => "Tower",
            0x8 => "Portable",
            0x9 => "Laptop",
            0xa => "Notebook",
            0xb => "Handheld",
            0xc => "Docking Station",
            0xd => "All-in-one",
            0xe => "Sub-notebook",
            0xf => "Space-saving",
            0x10 => "Lunch box",
            0x11 => "Main server chassis",
            0x12 => "Expansion chassis",
            0x13 => "SubChassis",
            0x14 => "Bus expansion chassis",
            0x15 => "Peripheral chassis",
            0x16 => "RAID chassis",
            0x17 => "Rack Mount Chassis",
            0x18 => "Sealed-case PC",
            0x19 => "Multi-system chassis",
            0x1a => "Compact PCI",
            0x1b => "Advanced TCA",
            0x1c => "Blade",
            0x1d => "Blade Enclosure",
            0x1e => "Tablet",
            0x1f => "Convertible",
            0x20 => "Detachable",
            0x21 => "IoT Gateway",
            0x22 => "Embedded PC",
            0x23 => "Mini PC",
            0x24 => "Stick PC",
            _ => "Unrecognized chassis type. Probably a bug.",
        };
        writeln!(f, "System Enclosure or Chassis Type: {}", t)?;
        Ok(())
    }

    pub fn fmt_chassis_version(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_str(f, 6, "Version")
    }

    pub fn fmt_chassis_serial(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_str(f, 7, "Serial Number")
    }

    pub fn fmt_chassis_asset_tag(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_str(f, 8, "Asset Tag")
    }

    fn fmt_state_byte(&self, b: usize) -> &str {
        match self.data.bits[b] {
            0x1 => "Other",
            0x2 => "Unknown",
            0x3 => "Safe",
            0x4 => "Warning",
            0x5 => "Critical",
            0x6 => "Non-recoverable",
            _ => "Unknown chassis state.",
        }
    }

    pub fn fmt_chassis_bootup_state(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t: &str = self.fmt_state_byte(9);
        writeln!(f, "System Enclosure or Chassis State: {}", t)
    }

    pub fn fmt_chassis_powersupply_state(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t: &str = self.fmt_state_byte(0xa);
        writeln!(f, "Power supply State: {}", t)
    }

    pub fn fmt_chassis_thermal_state(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t: &str = self.fmt_state_byte(0xb);
        writeln!(f, "Thermal State: {}", t)
    }

    pub fn fmt_chassis_security_status(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t: &str = match self.data.bits[0xc] {
            0x1 => "Other",
            0x2 => "Unknown",
            0x3 => "None",
            0x4 => "External interface locked out",
            0x5 => "External interface enabled",
            _ => "Unidentified status",
        };
        writeln!(f, "Chassis security status: {}", t)
    }

    pub fn fmt_chassis_height(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let h: String = match self.data.bits[0x11] {
            0 => String::from("Unspecified"),
            u => format!("{} U", u),
        };
        writeln!(f, "Chassis rack height: {}", h)
    }

    pub fn fmt_chassis_power_cords(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let h: String = match self.data.bits[0x12] {
            0 => String::from("Unspecified"),
            u => format!("{}", u),
        };
        writeln!(f, "Number of power cords: {}", h)
    }

    // TODO: handle full decoding of contained elements
    pub fn fmt_contained_elements(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let record_cnt = self.data.bits[0x13];
        let record_len = self.data.bits[0x14];
        let n_elements = record_cnt;
        let elem_head = 0x15;
        if record_cnt == 0 || record_len < 3 {
            // Per Table 16 – System Enclosure or Chassis (Type 3)
            // structure, record_len will be >=3 when elements are
            // present
            return writeln!(f, "Contained elements: 0");
        }
        writeln!(f, "Contained elements: {}", record_cnt)?;
        writeln!(f, "Element length: {}", record_len)?;
        let mut decode_element = |element_number, element_location: usize| {
            println!(
                "Decoding element {} at {}",
                element_number, element_location
            );
            if self.data.bits[element_location] & 128 > 0 {
                writeln!(
                    f,
                    "Contained element {} is an SMBIOS structure type {}. Decoding not yet implemented.",
                    element_number,
                    self.data.bits[element_location] & 127
                )?;
            } else {
                writeln!(f, "Contained element {} is an SMBIOS Baseboard type enumeration. Decoding not yet implemented.", element_number)?;
            }
            Ok(())
        };
        for i in 0..n_elements {
            decode_element(i, (elem_head + i * record_len).into())?;
        }
        Ok(())
    }

    pub fn fmt_chassis_sku(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let loc = 0x15 + self.data.bits[0x13] * self.data.bits[0x14];
        self.fmt_str(f, loc, "SKU")
    }

    pub fn fmt_table3(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Table 3 (System Enclosure or Chassis)")?;
        let len: u8 = self.size();
        self.fmt_chassis_manufacturer(f)?;
        self.decode_chassis_type_byte(f)?;
        self.fmt_chassis_version(f)?;
        self.fmt_chassis_serial(f)?;
        self.fmt_chassis_asset_tag(f)?;
        if len > 0x9 {
            self.fmt_chassis_bootup_state(f)?;
            self.fmt_chassis_powersupply_state(f)?;
            self.fmt_chassis_thermal_state(f)?;
            self.fmt_chassis_security_status(f)?;
            self.fmt_chassis_height(f)?;
            self.fmt_chassis_power_cords(f)?;
            self.fmt_contained_elements(f)?;
            self.fmt_chassis_sku(f)?;
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
    fn test_decode_table3() {
        let d = Data {
            location: 0,
            string_location: 0,
            next_loc: 0,
            bits: [
                3,   //type
                0xd, // length
                42,  // handle
                42,
                1,          // manufacturer string
                128 | 0x11, // type
                2,          // version string
                3,          // serial number string
                4,          // asset tag string
                3,          // boot up state
                3,          // power supply state
                3,          // thermal state
                5,          // security status
                0,
                0,
                0,
                0, // OEM defined DWORD
                2, // height
                2, // number of power cords
                0, // contained element count
                0, // contained element size
                5, // SKU string
            ]
            .to_vec(),
            strings: [
                String::from("ACME Widgets, Inc."),
                String::from("1.0"),
                String::from("12345"),
                String::from("my-asset-tag"),
                String::from("my-sku"),
            ]
            .to_vec(),
        };
        let table = Table {
            id: TableId::Chassis,
            data: d,
        };
        let r = format!("{}", table);
        println!("{}", r);
        assert!(r.contains("Manufacturer: ACME Widgets, Inc."));
        assert!(r.contains("Chassis lock is present"));
        assert!(r.contains("System Enclosure or Chassis Type: Main server chassis"));
        assert!(r.contains("SKU: my-sku"));
        assert!(r.contains("Chassis rack height: 2 U"));
        assert!(r.contains("Number of power cords: 2"));
    }

    #[test]
    fn test_decode_table3_with_contained_elements() {
        let d = Data {
            location: 0,
            string_location: 0,
            next_loc: 0,
            bits: [
                3,   //type
                0xd, // length
                42,  // handle
                42,
                1,          // manufacturer string
                128 | 0x11, // type
                2,          // version string
                3,          // serial number string
                4,          // asset tag string
                3,          // boot up state
                3,          // power supply state
                3,          // thermal state
                5,          // security status
                0,
                0,
                0,
                0, // OEM defined DWORD
                2, // height
                2, // number of power cords
                1, // contained element count
                3, // contained element size
                128,
                0,
                0,
                5, // SKU string
            ]
            .to_vec(),
            strings: [
                String::from("ACME Widgets, Inc."),
                String::from("1.0"),
                String::from("12345"),
                String::from("my-asset-tag"),
                String::from("my-sku"),
            ]
            .to_vec(),
        };
        let table = Table {
            id: TableId::Chassis,
            data: d,
        };
        let r = format!("{}", table);
        println!("{}", r);
        assert!(r.contains("Manufacturer: ACME Widgets, Inc."));
        assert!(r.contains("Chassis lock is present"));
        assert!(r.contains("System Enclosure or Chassis Type: Main server chassis"));
        assert!(r.contains("SKU: my-sku"));
        assert!(r.contains("Chassis rack height: 2 U"));
        assert!(r.contains("Number of power cords: 2"));
    }
}
