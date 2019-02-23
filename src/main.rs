#[macro_use]
extern crate clap;

use std::collections::HashSet;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::fs::File;

enum States {
    AwaitingBracket,
    // before the tree, waiting for a (
    Start,
    // start of processing, not in pattern
    Searching,
    // in what might be a pattern, searching to see if it is a pattern match
    NoWrite1,
    // pattern matched, waiting for a ,
    NoWrite2,          // pattern2 matched, waiting for a }
}

/// test States enums for equality
/// from: https://stackoverflow.com/questions/32554285/compare-enums-only-by-variant-not-value
fn state_eq(a: &States, b: &States) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

/// Writes the contents of pattern_buffer to the output and updates the pattern_counter and state.
///
/// This function exists to eliminate code duplication and is used when a pattern match fails
/// in Searching state
fn write_buffer(c: u8, pattern_buffer: &mut [u8], output_buffer: &mut BufWriter<File>, state: &mut States, pattern_counter: &mut usize) {
    pattern_buffer[*pattern_counter] = c;
    output_buffer.write_all(&pattern_buffer[0..=*pattern_counter]).expect("failed to write pattern buffer to output file");
    *state = States::Start;
    *pattern_counter = 0;
}

/// This code cleans up Gordon's BEAST trees that have the sequence embedded
/// what we want to remove is:
/// Eight_loc_Rec_regions_removed="[ACTG]+",
/// Eight_loc_Rec_regions_removed.set={[^}]+},
/// 123456789012345678901234567890123
/// 1         2         3         4
///
/// The code streams through the file, examining each character and applying
/// a finite state machine to decide when to switch from writing to not writing
fn main() {
    let matches = clap_app!(clean_tree =>
        (version: "0.2.0")
        (author: "Peter van Heusden <pvh@sanbi.ac.za>")
        (about: "Cleans up messy BEAST tree")
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
        output_buffer.write_all(buf).expect("write of initial read failed");
    }

    let mut bases: HashSet<u8> = HashSet::with_capacity(4);
    for b in [b'A', b'C', b'T', b'G'].iter() {
        bases.insert(*b);
    }

    let common_pattern = b"Eight_loc_Rec_regions_removed";
    let common_pattern_length = common_pattern.len();
    let pattern2 = b"Eight_loc_Rec_regions_removed.set";
    let pattern2_length = pattern2.len();
    let mut state: States = States::AwaitingBracket;
    let mut pattern_counter = 0;
    let mut pattern_buffer = [0; 34];
    for (byte_counter, c_maybe) in input_buffer.bytes().enumerate() {
        let c = c_maybe.unwrap(); // unwrap here because I don't expect read failures

        if state_eq(&state, &States::AwaitingBracket) {
            output_buffer.write_all(&[c]).expect("failed to write a character to output file");
            if c == b'(' {
                state = States::Start;
            }
        } else if state_eq(&state, &States::Start) {
            if c == common_pattern[0] {
                state = States::Searching;
                pattern_buffer[0] = c;
                pattern_counter += 1;
            } else {
                output_buffer.write_all(&[c]).expect("failed to write a character to output file");
            }
        } else if state_eq(&state, &States::NoWrite1) {
            // pattern seen - stop writing till we see a ,
            if c == b',' {
                state = States::Start;
            }
        } else if state_eq(&state, &States::NoWrite2) {
            if c == b'}' {
                state = States::NoWrite1;
            }
        } else if state_eq(&state, &States::Searching) {
            if pattern_counter < common_pattern_length {
                // we are in the common pattern
                if c != common_pattern[pattern_counter] {
                    // found a mismatch - write buffer and go to Start
                    write_buffer(c, &mut pattern_buffer, &mut output_buffer, &mut state, &mut pattern_counter);
                } else {
                    // still matching pattern, keeping putting data in pattern_buffer
                    pattern_buffer[pattern_counter] = c;
                    pattern_counter += 1;
                }
            } else if pattern_counter == common_pattern_length && c == b'=' {
                // pattern1 matched
                state = States::NoWrite1;
                pattern_counter = 0;
            } else if pattern_counter >= common_pattern_length && pattern_counter < pattern2_length {
                // didn't match pattern1 but might match pattern2
                if c == pattern2[pattern_counter] {
                    // still matching pattern2
                    pattern_buffer[pattern_counter] = c;
                    pattern_counter += 1;
                } else {
                    // pattern 2 match failed, write buffer and go to Start
                    write_buffer(c, &mut pattern_buffer, &mut output_buffer, &mut state, &mut pattern_counter);
                }
            } else if pattern_counter == pattern2_length {
                if c == b'=' {
                    // pattern 2 matched
                    state = States::NoWrite2;
                    pattern_counter = 0;
                } else {
                    // pattern 2 match failed, write buffer and go to Start
                    write_buffer(c, &mut pattern_buffer, &mut output_buffer, &mut state, &mut pattern_counter);
                }
            } else {
                panic!(format!("Got to unexpected state: {} {} {} {}", pattern_counter, pattern2_length, byte_counter, String::from_utf8_lossy(&pattern_buffer)));
            }
        }
    }
}

