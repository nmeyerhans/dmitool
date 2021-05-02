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

fn decode_bios_extension_byte1(data: &Vec<u8>) {
    let b: u8 = data[18];
    let bit_strings = [
        (1, "ACPI is supported"),
        (1 << 1, "USB Legacy is supported"),
        (1 << 2, "AGP is supported"),
        (1 << 3, "I2O boot is supported"),
        (1 << 4, "LS-120 SuperDisk boot is supported"),
        (1 << 5, "ATAPI ZIP drive boot is supported"),
        (1 << 6, "1394 boot is supported"),
        (1 << 7, "Smart batter is supported"),
    ];
    println!("Decoding BIOS Characteristics Extension byte 1:");
    for bit in bit_strings.iter() {
        if (b & bit.0) != 0 {
            println!("  + {}", bit.1);
        }
    }
}

fn decode_bios_extension_byte2(data: &Vec<u8>) {
    let b: u8 = data[19];
    let bit_strings = [
        (1, "BIOS Boot Specification is supported"),
        (1 << 1, "F-Key initiated network boot is supported"),
        (1 << 2, "Enable targeted content distribution"),
        (1 << 3, "UEFI Specification is supported"),
        (1 << 4, "SMBIOS table describes a virtual machine"),
        /* Remaining bits are reserved for future use */
    ];
    println!("Decoding BIOS Characteristics Extension byte 2:");
    for bit in bit_strings.iter() {
        if (b & bit.0) != 0 {
            println!("  + {}", bit.1);
        }
    }
}

fn decode_byte(b: u8, bit_strings: &[(u8, &str)]) {
    for bit in bit_strings.iter() {
        if (b & bit.0) != 0 {
            println!("  + {}", bit.1);
        }
    }
}

fn print_bios_table(table: &str, data: &Vec<u8>) {
    println!("Printing table {}", table);
    if data[0] != 0 {
        println!("Invalid byte 0 in BIOS characteristics table!");
        return;
    }
    let len: u8 = data[1];
    if len < 0x12 {
        println!("Invalid BIOS characteristics table length {}", len);
        return;
    }
    let pos = 0xa;
    if data[pos] & (1 << 3) != 0 {
        // Does "BIOS Characteristics are not supported" really mean we should skip this table?
        println!("BIOS Characteristics not supported on this system");
        return;
    }
    let bit_strings = [
        (1 << 4, "ISA is supported"),
        (1 << 5, "MCA is supported"),
        (1 << 6, "EISA is supported"),
        (1 << 7, "PCI is supported"),
    ];
    decode_byte(data[pos], &bit_strings);
    let pos = pos + 1;
    let bit_strings = [
        (1, "PCMCI is supported"),
        (1 << 1, "PnP is supported"),
        (1 << 2, "APM is supported"),
        (1 << 3, "BIOS upgrades are supported"),
        (1 << 4, "BIOS shadowing is allowed"),
        (1 << 5, "VL-VESA is supported"),
        (1 << 6, "ESCD support is available"),
        (1 << 7, "Boot from CD is supported"),
    ];
    decode_byte(data[pos], &bit_strings);
    let pos = pos + 1;
    let bit_strings = [
        (1, "Selectable boot is supported"),
        (1 << 1, "BIOS ROM is socketed"),
        (1 << 2, "Boot from PCMCIA (PC Card) is supported"),
        (1 << 3, "EDD Specification is supported"),
        (1 << 4, "Int 13h: NEC 9800 1.2 MB floppy is supported"),
        (1 << 5, "Int 13h: Toshiba 1.2 MB Floppy is supported"),
        (1 << 6, "Int 13h: 5.25” 360 KB floppy is supportd"),
        (1 << 7, "Int 13h: 5.25” 1.2 MB floppy is supported"),
    ];
    decode_byte(data[pos], &bit_strings);
    let pos = pos + 1;
    let bit_strings = [
        (1, "Int 13h: 3.5” / 720 KB floppy services are supported"),
        (
            1 << 1,
            "Int 13h: 3.5” / 2.88 MB floppy services are supported",
        ),
        (1 << 2, "Int 5h: print screen Service is supported"),
        (1 << 3, "Int 9h: 8042 keyboard services are supported"),
        (1 << 4, "Int 14h: serial services are supported"),
        (1 << 5, "Int 17h: printer services are supported"),
        (1 << 6, "Int 10h: CGA/Mono Video Services are supported"),
        (1 << 7, "NEC PC-98"),
    ];
    decode_byte(data[pos], &bit_strings);

    if len > 19 {
        decode_bios_extension_byte1(data);
    }
    if len > 20 {
        decode_bios_extension_byte2(data);
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
            Arg::with_name("raw")
                .short("r")
                .long("raw")
                .takes_value(false)
                .conflicts_with("zero")
                .help("read raw entrypoint"),
        )
        .get_matches();

    if matches.is_present("zero") {
        println!("Getting table zero");
        let table = "0-0";
        let res = read_table(&table);
        match res {
            Ok(table_data) => print_bios_table(&table, &table_data),
            Err(e) => println!("Reading table {}: {}", table, e),
        }
    } else if matches.is_present("raw") {
        println!("Will read from /sys/firmware/dmi/tables/smbios_entry_point");
        let t = match dmi::raw::decode_bios_raw_table() {
            Ok(t) => t,
            Err(e) => panic!("Unable to read raw table: {}", e),
        };
        println!("Got a {} table!", t.version());
    } else {
        print_vendor_data();
        print_product_data();
        print_system_data();
        print_bios_data();
    }
}
