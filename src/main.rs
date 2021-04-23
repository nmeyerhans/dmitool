use std::fs;
use std::io;
use std::path::PathBuf;

use clap::{App, Arg};

fn get_dmi_key(key: &str) -> Result<String, io::Error> {
    let id_root = "/sys/class/dmi/id";
    let path: PathBuf = [id_root, key].iter().collect();
    let r = String::from(fs::read_to_string(path)?.as_str().trim());
    Ok(r)
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
    } else {
        print_vendor_data();
        print_product_data();
        print_system_data();
        print_bios_data();
    }
}
