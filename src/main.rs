#[macro_use]
extern crate clap;
use std::collections::HashSet;
use std::io::{BufReader,BufWriter};
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let matches = clap_app!(clean_tree =>
        (version: "0.1.0")
        (author: "Peter van Heusden <pvh@sanbi.ac.za>")
        (about: "Cleans up messy tree")
        (@arg INPUT: +required "Input filename")
        (@arg OUTPUT: +required "Output filename")
    ).get_matches();

    let input_filename = matches.value_of("INPUT").expect("no input filename");
    let output_filename = matches.value_of("OUTPUT").expect("no output filename");

    println!("arguments: {} {}", input_filename, output_filename);

    let infile = File::open(input_filename).expect("can't open input file");
    let outfile = File::create(output_filename).expect("failed to create output file");

    let mut input_buffer = BufReader::new(infile);
    let mut output_buffer = BufWriter::new(outfile);
    {
        let buf: &mut [u8] = &mut [0; 48412];
        input_buffer.read_exact(buf).expect("read_exact failed");
        output_buffer.write(buf).expect("write of initial read failed");
    }

    let mut bases:HashSet<u8> = HashSet::with_capacity(4);
    for b in [b'A', b'C', b'T', b'G'].iter() {
        bases.insert(*b);
    }

    let mut seen_bracket = false;  // if we see the first ( we start filtering
    for c_maybe in input_buffer.bytes() {
        let c = c_maybe.unwrap();
        if !seen_bracket && c == b'(' {
            seen_bracket = true;
        }

        if !seen_bracket || !bases.contains( &c ) {  // copy the character to output if
                                                           //  we haven't seen a bracket yet
                                                           //  or otherwise if the character
                                                           //  is not a DNA base
            output_buffer.write(&[c]).expect("failed to write a character to output file");
        }
    }
//    loop {
//
//        let length = {
//            let buffer = input_buffer.fill_buf().unwrap();
//            for chunk in buffer.chunks(500) {
//                for c in chunk.iter() {
//
//                }
//            }
//            buffer.len()
//        };
//        if length == 0 {
//            break;
//        } else {
//            input_buffer.consume(length);
//        }
//    };
}
