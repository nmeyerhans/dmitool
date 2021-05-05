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
        (1 << 7, "Smart battery is supported"),
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

pub fn print_bios_table(table: &str, data: &Vec<u8>) {
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
