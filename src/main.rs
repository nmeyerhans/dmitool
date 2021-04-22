use std::fs;
use std::io;
use std::path::PathBuf;

fn get_dmi_key(key: &str) -> Result<String, io::Error> {
    let id_root = "/sys/class/dmi/id";
    let path: PathBuf = [id_root, key].iter().collect();
    let r = String::from(fs::read_to_string(path)?.as_str().trim());
    Ok(r)
}

fn print_vendor_data() {
    let dmi_info_name_keys = [
	("System", "sys_vendor"),
	("BIOS", "bios_vendor"),
	("Chassis", "chassis_vendor"),
	("Board", "board_vendor"),
    ];
    println!("Vendor information:");
    for dmi_name_key in dmi_info_name_keys.iter() {
	let sysfs_key = dmi_name_key.1;
	let data = get_dmi_key(&sysfs_key);
	let data = match data {
	    Ok(data) => data,
	    Err(e) => panic!("Couldn't read {}: {}", sysfs_key, e),
	};
	println!("  - {} is {}", dmi_name_key.0, data);
    }
}
fn main() {
    print_vendor_data();
}
