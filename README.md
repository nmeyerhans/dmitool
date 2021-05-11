# dmitool #

This is a simple DMI table decoder.  It reads and prints some data
from the
[SMBIOS](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.4.0.pdf)
firmware as exposed by Linux via the /sys/class/dmi/id and
/sys/firmware/dmi/entries interfaces.

**Usage:**
    
    DMI decoder tool 0.1.0
    Noah Meyerhans <frodo@morgul.net>
    Decodes and prints system information from the SMBIOS
    
    USAGE:
        dmitool [FLAGS] [OPTIONS]
    
    FLAGS:
        -e, --entrypoint    read SMBIOS entrypoint
        -h, --help          Prints help information
        -V, --version       Prints version information
        -0                  print table 0 via the /sys/firmware/dmi/entries interface
    
    OPTIONS:
        -t, --table <TABLE>    print the given table via the /sys/firmware/dmi/tables

**Example output (from a ThinkPad):**

    $ sudo ./target/debug/dmitool --table 0
    Table data:
    BIOS Characteristics
    Table handle is 17
      + PCI is supported
      + PnP is supported
      + BIOS upgrades are supported
      + BIOS shadowing is allowed
      + Boot from CD is supported
      + Selectable boot is supported
      + EDD Specification is supported
      + Int 13h: 3.5” / 720 KB floppy services are supported
      + Int 5h: print screen Service is supported
      + Int 9h: 8042 keyboard services are supported
      + Int 14h: serial services are supported
      + Int 17h: printer services are supported
      + Int 10h: CGA/Mono Video Services are supported
    BIOS Characteristics Extension byte 1:
      + ACPI is supported
      + USB Legacy is supported
    BIOS Characteristics Extension byte 2:
      + BIOS Boot Specification is supported
      + Enable targeted content distribution
      + UEFI Specification is supported
    BIOS Vendor: LENOVO
    BIOS Version: N2HET60W (1.43 )
    BIOS Release Date: 01/14/2021

This project is mostly an excuse for me to write Rust code while
digging in to the SMBIOS structures.  You probably don't want to use
this.  [dmidecode](https://nongnu.org/dmidecode/) is a much more
complete and mature DMI decoder and you should probably be using it if
you actully need to inspect your system's firmware.
