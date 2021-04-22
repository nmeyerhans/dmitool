use std::fs;
use std::io;
use std::path::PathBuf;

fn get_dmi_key(key: &str) -> Result<String, io::Error> {
    let id_root = "/sys/class/dmi/id";
    let path: PathBuf = [id_root, key].iter().collect();
    let r = String::from(fs::read_to_string(path)?.as_str().trim());
    Ok(r)
}

fn main() {
    let vendor_key = "sys_vendor";
    let vendor = get_dmi_key(&vendor_key);
    let vendor = match vendor {
	Ok(vendor) => vendor,
	Err(e) => panic!("nothing: {}", e),
    };
    println!("{} is {}", vendor_key, vendor)
}
