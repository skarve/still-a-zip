extern crate byteorder;
extern crate clap;

use clap::{App, Arg};
mod padzip;

fn main() {
    let matches = App::new("STiLLaZIP")
        .version("1.0")
        .author("Sanchit Karve <write2sanchit@gmail.com>")
        .about("Pads existing ZIP files with arbitrary data while maintaing conformance to ZIP file format specifications")
        .arg(
            Arg::with_name("input")
                .index(1)
                .requires("output")
                .required(true)
                .help("input file"),
        )
        .arg(
            Arg::with_name("output")
                .index(2)
                .required(true)
                .help("output file"),
        )
        .arg(
            Arg::with_name("prefix_string")
                .help("prefix string for ZIP")
                .takes_value(true)
                .short("s")
                .long("prefixstring"),
        )
        .arg(
            Arg::with_name("prefix_file")
                .help("file to use as prefix for ZIP")
                .takes_value(true)
                .short("f")
                .long("prefixfile")
                .conflicts_with("prefix_string"),
        )
        .get_matches();
    let input_filename = matches.value_of("input").unwrap();
    let output_filename = matches.value_of("output").unwrap();
    println!("ZIP to pad => {}", input_filename);
    println!("Padded ZIP => {}", output_filename);
    let pad_result = match matches.value_of("prefix_file") {
        Some(p_file) => {
            println!("Pad File   => {}", p_file);
            padzip::pad_zip_with_file(input_filename, p_file, output_filename)
        }
        None => {
            let prefix_str = matches
                .value_of("prefix_string")
                .unwrap_or("[PREFIX-PADDING-INSERTED-BY-STiLLaZIP]");
            println!("Pad String => {}", prefix_str);
            padzip::create_zip_with_prefix(input_filename, output_filename, prefix_str)
        }
    };
    match pad_result {
        Ok(total_mods) => println!("Padded ZIP Created! {} pointers modified", total_mods),
        Err(e) => println!("Error: {}", e),
    }
}
