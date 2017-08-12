use std::collections::HashMap;
use std::fmt;
use std::io::stdin;

fn main() {
    let text_adv = Adventure {
        stages: {
            let mut temp = HashMap::new();

            temp.insert("BEGIN".to_owned(), Stage {
                info: "The beginning.".to_owned(),
                nexts: vec![(String::new(), String::new()), ("foo".to_owned(), "Middle".to_owned())],
            });
            temp.insert("Middle".to_owned(), Stage {
                info: "The middle.".to_owned(),
                nexts: vec![("ix".to_owned(), String::new()), ("bar".to_owned(), "END".to_owned())],
            });
            temp
        },
    };

    text_adv.run();
}

struct Adventure {
    stages: HashMap<String, Stage>
}

impl Adventure {
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
                    let mut choice = String::new();
                    
                    stdin().read_line(&mut choice).unwrap();
                    
                    if let Ok(n) = choice.trim().parse::<usize>() {
                        break n;
                    } else {
                        eprintln!("That cannot be parsed as positive integer.");
                    }
                } - 1;

                if let Some(&(_, ref next)) = stage.nexts.get(choice) {
                    break self.stages.get(next);
                } else {
                    eprintln!("There aren't that many choices!");
                }
            }
        }
    }
}

struct Stage {
    info: String,
    nexts: Vec<(String, String)>,
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.info,
            self.nexts.iter().enumerate()      //    yellow    white
                .map(|(i, &(ref c, _))| format!("    {}[33m[{}]{}[37m {}\n", 27 as char, &i + 1, 27 as char, c))
                .collect::<String>()
        )
    }
}
