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
        pub fn fmt_table2(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Table 2 (Baseboard Information)\n")?;
            self.fmt_baseboard_manufacturer(f)?;
            self.fmt_baseboard_product(f)?;
            self.fmt_baseboard_version(f)?;
            self.fmt_baseboard_serial(f)?;
            self.fmt_baseboard_asset_tag(f)?;
            // TODO: format additional strings, bitfields, etc
            Ok(())
        }
    }
}
