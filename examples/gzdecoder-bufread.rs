extern crate flate2;

use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::bufread::GzDecoder;

// Compress a sample string and print it after transformation.
fn main() {
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write(b"Hello World").unwrap();
    let bytes = e.finish().unwrap();
    println!("{}", decode_reader(bytes).unwrap());
}

// Uncompresses a Gz Encoded vector of bytes and returns a string or error
// Here &[u8] implements BufRead
fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
    let mut gz = GzDecoder::new(&bytes[..])?;
    let mut s = String::new();
    gz.read_to_string(&mut s)?;
    Ok(s)
}
