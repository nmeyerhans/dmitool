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
        dmitool [FLAGS]

    FLAGS:
        -h, --help       Prints help information
        -r, --raw        read raw entrypoint
        -V, --version    Prints version information
        -0               print table 0


This project is mostly an excuse for me to write Rust code while
digging in to the SMBIOS structures.  You probably don't want to use
this.  [dmidecode](https://nongnu.org/dmidecode/) is a much more
complete and mature DMI decoder and you should probably be using it if
you actully need to dig in to your system's firmware.

