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

use std::fs;
use std::io;
use std::path::PathBuf;

use clap::{App, Arg};

const DMI_ID_ROOT: &str = "/sys/class/dmi/id";
const DMI_ENTRIES_ROOT: &str = "/sys/firmware/dmi/entries";

fn get_dmi_key(key: &str) -> Result<String, io::Error> {
    let path: PathBuf = [DMI_ID_ROOT, key].iter().collect();
    let r = String::from(fs::read_to_string(path)?.as_str().trim());
    Ok(r)
}

fn read_table(id: &str) -> Result<Vec<u8>, io::Error> {
    let root: PathBuf = [DMI_ENTRIES_ROOT, id].iter().collect();
    let rawpath: PathBuf = root.join("raw");
    println!("Reading table from {}", rawpath.as_path().to_str().unwrap());
    fs::read(rawpath.as_path())
}

fn print_dmi_id_fields(dmi_info_name_keys: &[(&str, &str)]) {
    for dmi_name_key in dmi_info_name_keys.iter() {
        let sysfs_key = dmi_name_key.1;
        let data = get_dmi_key(&sysfs_key);
        match data {
            Ok(data) => println!("  - {} is {}", dmi_name_key.0, data),
            Err(e) => println!("  * Error reading {}: {}", dmi_name_key.0, e),
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
    println!("Vendor information:");
    print_dmi_id_fields(&dmi_info_name_keys);
}

fn print_system_data() {
    let keys = [("Vendor", "sys_vendor")];
    println!("System data:");
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

    println!("Product information:");
    print_dmi_id_fields(&dmi_info_name_keys);
}

fn print_bios_data() {
    let keys = [
        ("Date", "bios_date"),
        ("Release", "bios_date"),
        ("Vendor", "bios_vendor"),
        ("Version", "bios_version"),
    ];
    println!("BIOS Information");
    print_dmi_id_fields(&keys);
}

fn do_entrypoint() {
    let t = match dmi::raw::decode_entrypoint() {
        Ok(t) => t,
        Err(e) => panic!("Unable to read entrypoint: {}", e),
    };
    println!("Found a {} entrypoint!", t.version());
}

fn do_table() {
    let t = match dmi::raw::print_raw_table() {
        Ok(t) => t,
        Err(e) => panic!("Unable to read table: {}", e),
    };
    println!(
        "Got a table with ID 0x{:02x} and handle 0x{:04x}",
        t.id(),
        t.handle()
    );
    for s in t.strings().iter() {
        println!("Table has string [{}]", s);
    }
}

fn main() {
    let matches = App::new("DMI decoder tool")
        .version("0.1.0")
        .author("Noah Meyerhans <frodo@morgul.net>")
        .about("Decodes and prints system information from the SMBIOS")
        .arg(
            Arg::with_name("zero")
                .short("0")
                .takes_value(false)
                .help("print table 0"),
        )
        .arg(
            Arg::with_name("table")
                .short("t")
                .long("table")
                .takes_value(false)
                .conflicts_with("zero")
                .help("read raw BIOS table"),
        )
        .arg(
            Arg::with_name("entrypoint")
                .short("e")
                .long("entrypoint")
                .takes_value(false)
                .conflicts_with("zero")
                .help("read SMBIOS entrypoint"),
        )
        .get_matches();

    if matches.is_present("zero") {
        println!("Getting table zero");
        let table = "0-0";
        let res = read_table(&table);
        match res {
            Ok(table_data) => dmi::decode::print_bios_table(&table, &table_data),
            Err(e) => println!("Reading table {}: {}", table, e),
        }
    } else if matches.is_present("entrypoint") {
        do_entrypoint();
    } else if matches.is_present("table") {
        do_table();
    } else {
        print_vendor_data();
        print_product_data();
        print_system_data();
        print_bios_data();
    }
}
