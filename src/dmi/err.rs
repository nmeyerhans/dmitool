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

#[derive(Debug)]
pub enum DMIParserError {
    HeaderDataError,
    IOError(std::io::Error),
}

impl std::error::Error for DMIParserError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            DMIParserError::HeaderDataError => None,
            DMIParserError::IOError(ref e) => Some(e),
        }
    }
}

impl std::fmt::Display for DMIParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DMIParserError::HeaderDataError => write!(f, "Header error"),
            DMIParserError::IOError(ref e) => write!(f, "IOError: {}", e),
        }
    }
}

impl From<std::io::Error> for DMIParserError {
    fn from(error: std::io::Error) -> Self {
        DMIParserError::IOError(error)
    }
}
