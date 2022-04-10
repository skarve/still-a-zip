# still-a-zip
A command-line Rust tool to create valid ZIP files that bypass most ZIP file type detection programs.

# Why did you make this?
Far too many people write production code that expect ZIP files to have the magic bytes "PK\x03\x04" at the beginning of the file (BOF). However, the magic bytes for ZIP actually lie near the end-of-file (EOF).
This results in many valid ZIPs being incorrectly flagged as data or something else, and allows malicious actors to take advantage of it in a number of ways.
I've been learning Rust recently and figured writing a tool in Rust would be a nice way to get more comfortable with Rust and work on something fun.

# Usage
You can use either a string or any data for padding.
Mix and match filetypes that have markers at/near BOF and EOF for best results.

For example:

* You can use an existing ZIP file and choose a PDF file for the padding content. The resulting file is both a valid ZIP and PDF file.
* This file can be padded with a small image type (PNG) to generate a file that is a valid ZIP, PDF and PNG at the same time.

```
Sanchit Karve <write2sanchit@gmail.com>
Pads existing ZIP files with arbitrary data while maintaing conformance to ZIP file format specifications

USAGE:
    still-a-zip.exe [OPTIONS] <input> <output>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --prefixfile <prefix_file>        file to use as prefix for ZIP
    -s, --prefixstring <prefix_string>    prefix string for ZIP

ARGS:
    <input>     input file
    <output>    output file
```

# Supported Rust Versions

Compiles with Rust 2018 and 2021 editions. I've also included three basic tests.