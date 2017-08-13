extern crate text_adv;
use text_adv::Adventure;

use std::fs::File;
use std::io::{Read, BufReader};

fn main() {
    let file = std::env::args().skip(1).next()
        .expect("A filename is required as the first argument");


    let file = File::open(file)
        .expect("Unable to open file");
    
    let mut text = String::new();
    
    BufReader::new(file).read_to_string(&mut text)
        .expect("Could not read file as string");

    let text_adv = Adventure::read_from(&text);
    
    text_adv.run();
}
