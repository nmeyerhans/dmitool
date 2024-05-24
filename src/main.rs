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

mod dmi;

use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::path::PathBuf;

use crate::dmi::table::Table;

#[macro_use]
extern crate log;

use env_logger::Env;

use clap::{Command, Arg};

const DMI_ID_ROOT: &str = "/sys/class/dmi/id";
const DMI_ENTRIES_ROOT: &str = "/sys/firmware/dmi/entries";

fn get_dmi_key(key: &str) -> Result<String, io::Error> {
    let path: PathBuf = [DMI_ID_ROOT, key].iter().collect();
    let r = String::from(fs::read_to_string(path)?.as_str().trim());
    Ok(r)
}

fn read_table(id: &str) -> Result<Table, dmi::err::DMIParserError> {
    let root: PathBuf = [DMI_ENTRIES_ROOT, id].iter().collect();
    let rawpath: PathBuf = root.join("raw");
    debug!("Reading table from {}", rawpath.as_path().to_str().unwrap());
    Table::read_fh_at(File::open(rawpath.as_path())?, 0)
}

fn print_dmi_id_fields(dmi_info_name_keys: &[(&str, &str)]) {
    for dmi_name_key in dmi_info_name_keys.iter() {
        let sysfs_key = dmi_name_key.1;
        let data = get_dmi_key(sysfs_key);
        match data {
            Ok(data) => println!("  - {} is {}", dmi_name_key.0, data),
            Err(e) => panic!("  * Error reading {}: {}", dmi_name_key.0, e),
        };
    }
}

fn print_vendor_data() {
    let dmi_info_name_keys = [
        ("System", "sys_vendor"),
        ("BIOS", "bios_vendor"),
        ("Chassis", "chassis_vendor"),
        ("Board", "board_vendor"),
    ];
    info!("Vendor information:");
    print_dmi_id_fields(&dmi_info_name_keys);
}

fn print_system_data() {
    let keys = [("Vendor", "sys_vendor")];
    info!("System data:");
    print_dmi_id_fields(&keys);
}

fn print_product_data() {
    let dmi_info_name_keys = [
        ("Family", "product_family"),
        ("Name", "product_name"),
        ("Serial", "product_serial"),
        ("SKU", "product_sku"),
        ("UUID", "product_uuid"),
        ("Version", "product_version"),
    ];

    info!("Product information:");
    print_dmi_id_fields(&dmi_info_name_keys);
}

fn print_bios_data() {
    let keys = [
        ("Date", "bios_date"),
        ("Release", "bios_date"),
        ("Vendor", "bios_vendor"),
        ("Version", "bios_version"),
    ];
    info!("BIOS Information");
    print_dmi_id_fields(&keys);
}

fn do_entrypoint() {
    let t = match dmi::raw::decode_entrypoint() {
        Ok(t) => t,
        Err(e) => panic!("Unable to read entrypoint: {}", e),
    };
    info!("Found a {} entrypoint!", t.version());
}

fn do_table(id: u8) {
    let entrypoint = match dmi::entrypoint::Entrypoint::read() {
        Ok(t) => t,
        Err(e) => panic!("Unable to read entrypont: {}", e),
    };
    let t = match dmi::raw::read_raw_table(id, entrypoint) {
        Ok(t) => t,
        Err(e) => panic!("Unable to read table: {}", e),
    };
    debug!(
        "Got a table with ID 0x{:02x} and handle 0x{:04x}",
        t.id(),
        t.handle()
    );
    for s in t.strings().iter() {
        debug!("Table has string [{}]", s);
    }
}

fn main() {
    let args = Command::new("DMI decoder tool")
        .version("0.1.0")
        .author("Noah Meyerhans <frodo@morgul.net>")
        .about("Decodes and prints system information from the SMBIOS")
        .arg(
            Arg::new("zero")
                .short('0')
                .takes_value(false)
                .help("print table 0 via the /sys/firmware/dmi/entries interface"),
        )
        .arg(
            Arg::new("table")
                .short('t')
                .long("table")
                .takes_value(true)
                .value_name("TABLE")
                .conflicts_with("zero")
                .help("print the given table via the /sys/firmware/dmi/tables"),
        )
        .arg(
            Arg::new("entrypoint")
                .short('e')
                .long("entrypoint")
                .takes_value(false)
                .conflicts_with("zero")
                .help("read SMBIOS entrypoint"),
        )
        .arg(
	    Arg::new("debug")
		.short('d')
		.long("debug")
		.takes_value(false)
		.help("enable debug output")
	)
	.get_matches();

    if args.contains_id("debug") {
	env::set_var("LOG_LEVEL", "debug")
    }

    let env = Env::default()
        .filter_or("LOG_LEVEL", "info")
        .write_style_or("LOG_STYLE", "never");

    env_logger::init_from_env(env);

    if args.contains_id("zero") {
        info!("Getting table zero");
        let table = "0-0";
        let res = read_table(table);
        match res {
            Ok(t) => print!("Table {}\n{}", &table, &t),
            Err(e) => panic!("Reading table {}: {}", table, e),
        }
    } else if args.contains_id("entrypoint") {
        do_entrypoint();
    } else if args.contains_id("table") {
        let table_id: u8 = match args.get_one::<String>("table").unwrap().parse() {
            Ok(t) => t,
            Err(_e) => panic!("unable to parse table ID"),
        };
        do_table(table_id);
    } else {
        print_vendor_data();
        print_product_data();
        print_system_data();
        print_bios_data();
    }
}
