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

use crate::dmi;
use crate::dmi::entrypoint;
use crate::dmi::err;
use crate::dmi::table;

pub fn decode_entrypoint() -> Result<entrypoint::Entrypoint, err::DMIParserError> {
    let t = entrypoint::Entrypoint::read()?;
    Ok(t)
}

pub fn print_raw_table() -> Result<table::Table, err::DMIParserError> {
    let mut t = table::Table::read()?;
    println!(
        "Read table at position 0, next is at position 0x{:x}",
        t.next_loc
    );

    for i in 0..20 {
        t = table::Table::read_at(t.next_loc)?;
        println!(
            "Read table at position 0x{:x}, ID 0x{:02x}, Handle 0x{:04x}",
            t.location,
            t.id(),
            t.handle()
        );
        if t.id() == 0 {
            println!("Found table 0!");
            dmi::decode::print_bios_table("zero", &t.bits);
            break;
        }
    }
    Ok(t)
}
