extern crate reducto;

use reducto::lz77::LZ77;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() {
    // example from https://www.researchgate.net/figure/An-example-of-LZ77-encoding_fig4_322296027
    let test_sample = "aacaacabcabaaac";
    let mut lz77 = LZ77::new(test_sample, 4, 6);
    lz77.encode();

    let decoded = lz77.decode();
    println!("Test String   : {}", test_sample);
    println!("Decoded String: {}", decoded);
    assert_eq!(test_sample, decoded);

    let test_sample = "LZ77 présente certains défauts, en particulier";
    let mut lz77 = LZ77::new(test_sample, 4, 6);
    lz77.encode();

    let decoded = lz77.decode();
    println!("Test String   : {}", test_sample);
    println!("Decoded String: {}", decoded);
    assert_eq!(test_sample, decoded);

    let path = Path::new("resources/small_lorem.txt");
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };
    let mut test_sample = String::new();
    let mut reader = BufReader::new(file);
    if let Err(e) = reader.read_to_string(&mut test_sample) {
        panic!("{}", e)
    }

    let mut lz77 = LZ77::new(&test_sample, 16, 32);
    lz77.encode();

    let decoded = lz77.decode();
    println!("Test String   : {}", test_sample);
    println!("Decoded String: {}", decoded);
    assert_eq!(test_sample, decoded);

    // test with utf code
    let test_sample = "網站有中、英文版本，也有繁、簡體版，可通過每頁左上角的連結隨時調整。";
    let mut lz77 = LZ77::new(test_sample, 8, 16);
    lz77.encode();

    let decoded = lz77.decode();
    println!("Test String   : {}", test_sample);
    println!("Decoded String: {}", decoded);
    assert_eq!(test_sample, decoded);

    let path = Path::new("resources/lorem.txt");
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };
    let mut test_sample = String::new();
    let mut reader = BufReader::new(file);
    if let Err(e) = reader.read_to_string(&mut test_sample) {
        panic!("{}", e)
    }

    let mut lz77 = LZ77::new(&test_sample, 16, 32);
    lz77.encode();

    let decoded = lz77.decode();
    println!("Test String   : {}", test_sample);
    println!("Decoded String: {}", decoded);
    assert_eq!(test_sample, decoded);
}
