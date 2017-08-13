#[macro_use]
extern crate lazy_static;

extern crate regex;
use regex::{Regex, RegexBuilder};

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Adventure<'a> {
    stages: HashMap<&'a str, Stage<'a>>
}

impl <'a> Adventure<'a> {
    pub fn read_from(source: &'a str) -> Adventure<'a> {
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

    pub fn get(&self, stage: &str) -> Option<&Stage<'a>> { self.stages.get(stage) }
}

pub struct Adventurer<'a> {
    adventure: &'a Adventure<'a>,
    choices: &'a Vec<(&'a str, &'a str)>,
}

impl <'a> Adventurer<'a> {
    pub fn new(adventure: &'a Adventure<'a>, begin: &'a Stage<'a>) -> Adventurer<'a> {
        Adventurer {
            adventure,
            choices: &begin.nexts,
        }
    }

    pub fn choose(&mut self, choice: usize) -> Result<Option<&Stage>, InvalidChoiceError> {
        match self.choices.get(choice) {
            Some(&(_, s)) => {
                let new_stage = self.adventure.get(s);

                if let Some(stage) = new_stage {
                    self.choices = &stage.nexts;
                }

                Ok(new_stage)
            },
            None    => Err(InvalidChoiceError {
                number_chosen: choice + 1,
                choice_number: self.choices.len(),
            }),
        }
    }
}

#[derive(Debug)]
pub struct InvalidChoiceError {
    number_chosen: usize,
    choice_number: usize,
}

impl Error for InvalidChoiceError {
    fn description(&self) -> &str {
        "There aren't that many choices"
    }
}

impl fmt::Display for InvalidChoiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Choice #{} was picked when only {} were available",
            self.number_chosen, self.choice_number)
    }
}

#[derive(Debug)]
pub struct Stage<'a> {
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

