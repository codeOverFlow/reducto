extern crate reducto;

use reducto::huffman::Huffman;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() {
    let path = Path::new("resources/lorem.txt");
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };
    let mut buffer = String::new();
    let mut reader = BufReader::new(file);
    if let Err(e) = reader.read_to_string(&mut buffer) {
        panic!("{}", e)
    }

    let mut encoder: Huffman = Huffman::new();
    let encoded = encoder.encode(&buffer);
    println!("ENCODER: {:#?}", encoder);
    println!("ENCODED: {:#?}", encoded);

    let decoded = encoder.decode(&encoded);
    println!("DECODED: {:#?}", decoded);
    // let compressed_data: Vec<u8> = Gzip::from("str");
    // let compressed_data: Vec<u8> = Gzip::from(String::new());
    // let compressed_data: Vec<u8> = Gzip::from(b"oneu");
    // let compressed_data: Vec<u8> = Gzip::from(File);
    //
    // let result: io::Result<()> = Gzip::compress_to_file("path", "file.gzip");
}
