# dmitool #

This is a simple DMI table decoder.  It reads and prints some data
from the
[SMBIOS](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.4.0.pdf)
firmware as exposed by Linux via the /sys/class/dmi/id and
/sys/firmware/dmi/entries interfaces.

    Usage:
    
	DMI decoder tool 0.1.0
	Noah Meyerhans <frodo@morgul.net>
	Decodes and prints system information from the SMBIOS

	USAGE:
		dmitool [FLAGS] [OPTIONS]

	FLAGS:
		-e, --entrypoint    read SMBIOS entrypoint
		-h, --help          Prints help information
		-V, --version       Prints version information
		-0                  print table 0

	OPTIONS:
		-t, --table <TABLE>    read the given BIOS table

Example usage (from a ThinkPad):

    $ sudo ./target/debug/dmitool --table 0
    Read table at position 0, next is at position 0x48
    Read table at position 0x48, ID 0x0e, Handle 0x0001
    Read table at position 0x72, ID 0x10, Handle 0x0002
    Read table at position 0x8b, ID 0x11, Handle 0x0003
    Read table at position 0xf6, ID 0x11, Handle 0x0004
    Read table at position 0x161, ID 0x13, Handle 0x0005
    Read table at position 0x182, ID 0xdd, Handle 0x0006
    Read table at position 0x19a, ID 0xdd, Handle 0x0007
    Read table at position 0x1e8, ID 0xdd, Handle 0x0008
    Read table at position 0x245, ID 0xdd, Handle 0x0009
    Read table at position 0x3a7, ID 0xdd, Handle 0x000a
    Read table at position 0x477, ID 0xdd, Handle 0x000b
    Read table at position 0x497, ID 0x07, Handle 0x000c
    Read table at position 0x4bc, ID 0x07, Handle 0x000d
    Read table at position 0x4e1, ID 0x07, Handle 0x000e
    Read table at position 0x506, ID 0x04, Handle 0x000f
    Read table at position 0x589, ID 0x86, Handle 0x0010
    Read table at position 0x598, ID 0x00, Handle 0x0011
    Found table 0!
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
    Got a table with ID 0x00 and handle 0x0011
    Table has string [LENOVO]
    Table has string [N2HET60W (1.43 )]
    Table has string [01/14/2021]

This project is mostly an excuse for me to write Rust code while
digging in to the SMBIOS structures.  You probably don't want to use
this.  [dmidecode](https://nongnu.org/dmidecode/) is a much more
complete and mature DMI decoder and you should probably be using it if
you actully need to inspect your system's firmware.
