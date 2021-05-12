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

    pub fn fmt_table1(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len: u8 = self.data.bits[1];
        // SMBIOS 2.0 uses len 0x8
        // SMBIOS 2.1-2.3.4 use len 0x19
        // Newer versions (2.4+) use len 0x1b
        self.fmt_manufacturer(f)?;
        self.fmt_product_name(f)?;
        self.fmt_product_version(f)?;
        self.fmt_product_serial(f)?;
        if len >= 0x1b {
            self.fmt_sku(f)?;
            self.fmt_family(f)?;
        }
        // TODO: decode the system UUID and wake-up timer byte
        Ok(())
    }
}
