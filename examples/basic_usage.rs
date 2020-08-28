extern crate reducto;

use reducto::gzip::GZip;

fn main() {
    GZip::decompress_file("/home/codeoverflow/toto.gz");
    // let compressed_data: Vec<u8> = Gzip::from("str");
    // let compressed_data: Vec<u8> = Gzip::from(String::new());
    // let compressed_data: Vec<u8> = Gzip::from(b"oneu");
    // let compressed_data: Vec<u8> = Gzip::from(File);
    //
    // let result: io::Result<()> = Gzip::compress_to_file("path", "file.gzip");
}
