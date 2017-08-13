extern crate text_adv;
use text_adv::{Adventure, Adventurer};

use std::fs::File;
use std::error::Error;
use std::io::{stdin, Read, BufReader};

fn main() {
    let file = std::env::args().skip(1).next()
        .expect("A filename is required as the first argument");
    
    
    let file = File::open(file)
        .expect("Unable to open file");
    
    let mut text = String::new();
    
    BufReader::new(file).read_to_string(&mut text)
        .expect("Could not read file as string");
    
    let adventure = Adventure::read_from(&text);

    let mut adventurer = Adventurer::new(&adventure, {
        let beginning = adventure.get("BEGIN")
            .expect("That adventure has no beginning");
        
        println!("{}", beginning);

        beginning
    });
    
    loop {
        match adventurer.choose(loop {
            let mut choice = String::new();
            
            if let Err(_) = stdin().read_line(&mut choice) {
                eprintln!("A line could not be read.");
                continue;
            }
            
            if let Ok(n) = choice.trim().parse::<usize>() {
                break n - 1;
            } else {
                eprintln!("That cannot be parsed as positive integer.");
            }
        })
        {
            Ok(Some(v)) => println!("{}", v),
            Ok(None) => {
                println!("\n\n    The End.\n\n");
                
                break;
            },
            Err(e) => eprintln!("{}", e.description()),
        }
    }
}
