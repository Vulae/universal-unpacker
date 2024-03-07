
# [universal-unpacker](https://github.com/Vulae/universal-unpacker)

A simple command line tool to extract assets from a variety of archival formats. (Mainly game archives)

## [Formats](#formats)

* Godot [.pck](https://github.com/godotengine/godot/blob/master/core/io/file_access_pack.cpp#L130) file
    * Automatically convert [textures](https://github.com/godotengine/godot/blob/master/editor/import/resource_importer_texture.cpp#L257) to compatible image formats
    * Extract resource data
* RenPy [.rpa](https://github.com/renpy/renpy/blob/master/renpy/loader.py#L101) file
    * Very basic compiled script decompilation
* Source engine [.vpk]((https://developer.valvesoftware.com/wiki/VPK_(file_format))) file

A list of to be implemented formats is in the [`TODO.md`](/TODO.md#future-unpackers)

## [Usage](#usage)

* To extract an archive use `universal-unpacker extract "path/to/output/" *unpacker* "path/to/archive"`
* use `universal-unpacker extract --help` for extract options and list of unpackers.
* use `universal-unpacker extract *unpacker* --help` for specific unpacker options.

## [`MIT License`](/LICENSE)
