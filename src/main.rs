#[macro_use]
extern crate lazy_static;

extern crate regex;
use regex::{Regex, RegexBuilder};

use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{stdin, Read, BufReader};

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

#[derive(Debug)]
struct Adventure<'a> {
    stages: HashMap<&'a str, Stage<'a>>
}

impl <'a> Adventure<'a> {
    fn read_from(source: &'a str) -> Adventure<'a> {
        lazy_static! {
            static ref PATTERN: Regex = RegexBuilder::new(r"\s*(@|-)\s*`(.*?)`\s*`(.*?)`")
                .multi_line(true)
                .dot_matches_new_line(true)
                .build().unwrap();
        }
        
        let mut adventure = Adventure {
            stages: HashMap::<&'a str, Stage<'a>>::new(),
        };
        
        let mut stage_id = None;
        let mut stage: Option<Stage<'a>> = None;
        
        for s in source.split(',') {
            if let Some(s) = PATTERN.captures(s) {
                match s.get(1).unwrap().as_str() {
                    "@" => {
                        if let Some(stage) = stage {
                            adventure.stages.insert(stage_id.unwrap(), stage);
                        }

                        stage_id = Some(s.get(2).unwrap().as_str());
                        stage = Some(Stage {
                            info: s.get(3).unwrap().as_str(),
                            nexts: Vec::new(),
                        });
                    },
                    "-" => if let Some(ref mut stage) = stage {
                        stage.nexts.push((s.get(2).unwrap().as_str(), s.get(3).unwrap().as_str()));
                    },
                    _ => unreachable!(),
                }
            }
        }
        if let Some(stage) = stage {
            adventure.stages.insert(stage_id.unwrap(), stage);
        }

        adventure
    }

    fn run(&self) {
        let mut stage_next = self.stages.get("BEGIN"); 
        
        loop {
            let stage = match stage_next {
                Some(s) => {
                    println!("{}", s);
                    s
                },
                None    => {
                    println!("\n\n    The End.\n\n");
                    break;
                },
            };

            stage_next = loop {
                let choice = loop {
                    println!("Which option do you choose?");

                    let mut choice = String::new();
                    
                    if let Err(_) = stdin().read_line(&mut choice) {
                        eprintln!("A line could not be read.");
                        continue;
                    }
                    
                    if let Ok(n) = choice.trim().parse::<usize>() {
                        break n;
                    } else {
                        eprintln!("That cannot be parsed as positive integer.");
                    }
                } - 1;

                if let Some(&(_, next)) = stage.nexts.get(choice) {
                    break self.stages.get(next);
                } else {
                    eprintln!("There aren't that many choices!");
                }
            }
        }
    }
}

#[derive(Debug)]
struct Stage<'a> {
    info: &'a str,
    nexts: Vec<(&'a str, &'a str)>,
}

impl <'a> fmt::Display for Stage<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.info,
            self.nexts.iter().enumerate()      //yellow    white
                .map(|(i, &(c, _))| format!("    {}[33m[{}]{}[37m {}\n", 27 as char, &i + 1, 27 as char, c))
                .collect::<String>()
        )
    }
}
