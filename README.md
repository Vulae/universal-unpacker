
# [universal-unpacker](/)

A simple command line tool to extract assets from a variety of archival formats. (Mainly game archives)

## [Formats](#formats)

* Godot .pck file
    * Automatically convert textures to compatible image formats
    * Extract resource data
* RenPy .rpa file

A list of to be implemented formats is in the [`TODO.md`](/TODO.md#future-unpackers)

## [Usage](#usage)

To extract an archive use
`universal-unpacker extract "path/to/output/" *unpacker* "path/to/archive"`
use `universal-unpacker extract --help` for extract options and list of unpackers.
use `universal-unpacker extract *unpacker* --help` for specific unpacker options.

## [`MIT License`](/LICENSE)
