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

fn print_raw_table(table: &str, data: &Vec<u8>) {
    println!("Printing table {}", table);
    if data.len() > 19 {
        decode_bios_extension_byte1(data);
    }
    if data.len() > 20 {
        decode_bios_extension_byte2(data);
    }
}

fn main() {
    let matches = App::new("My Test Program")
        .version("0.1.0")
        .author("Noah Meyerhans <frodo@morgul.net>")
        .about("Prints system information")
        .arg(
            Arg::with_name("zero")
                .short("0")
                .takes_value(false)
                .help("print table 0"),
        )
        .get_matches();

    if matches.is_present("zero") {
        println!("Getting table zero");
        let table = "0-0";
        let res = read_table(&table);
        match res {
            Ok(table_data) => print_raw_table(&table, &table_data),
            Err(e) => println!("Reading table {}: {}", table, e),
        }
    } else {
        print_vendor_data();
        print_product_data();
        print_system_data();
        print_bios_data();
    }
}
