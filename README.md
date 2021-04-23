# dmitool #

This is a simple DMI table decoder.  It reads and prints some data
from the
[SMBIOS](https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.4.0.pdf)
firmware as exposed by Linux via the /sys/class/dmi/id and
/sys/firmware/dmi/entries interfaces.

This project is mostly an excuse for me to write Rust code while
digging in to the SMBIOS structures.  You probably don't want to use
this.  [dmidecode](https://nongnu.org/dmidecode/) is a much more
complete and mature DMI decoder and you should probably be using it if
you actully need to dig in to your system's firmware.

