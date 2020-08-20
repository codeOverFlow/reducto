extern crate reducto;

use reducto::counter::Counter;
use std::fs::File;
use std::io::{BufReader, Read};
use std::iter::FromIterator;
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

    // load the text into the counter
    let mut counter: Counter<char> = Counter::from_iter(buffer.chars());

    // debug print
    println!("{:#?}", counter);

    // use in for loop
    for (key, count) in &counter {
        println!("{} = {}", key, count);
    }

    // indexing
    println!("Count for c: {}", counter['c']);

    // update with iter
    counter.update_from_iter("ccc".chars());
    println!("Count for c: {}", counter['c']);

    // update with value
    counter.update_from_value('c');
    println!("Count for c: {}", counter['c']);
}
