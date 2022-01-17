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

use crate::dmi::entrypoint;
use crate::dmi::err;
use crate::dmi::table::Table;

pub fn decode_entrypoint() -> Result<entrypoint::Entrypoint, err::DMIParserError> {
    let t = entrypoint::Entrypoint::read()?;
    Ok(t)
}

pub fn read_raw_table(
    id: u8,
    entrypoint: entrypoint::Entrypoint,
) -> Result<Table, err::DMIParserError> {
    let table_size = entrypoint.table_size();
    let mut t = Table::read()?;

    for _i in 0..1000 {
        debug!(
            "Read table at position 0x{:x}, ID 0x{:02x}, Handle 0x{:04x}, Size 0x{:04x}",
            t.location(),
            t.id(),
            t.handle(),
            t.size(),
        );
        if t.id() == 127 {
            warn!("Found End-of-table structure");
            break;
        }
        if t.id() == id {
            debug!("Found table {}!", id);
            print!("{}", &t);
            break;
        }
        if t.next_loc() > table_size.into() {
            warn!("Reached end of table");
            break;
        }
        t = Table::read_at(t.next_loc())?;
    }
    Ok(t)
}
