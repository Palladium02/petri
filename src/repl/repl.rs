use std::{
    io::{self, Write},
    process,
};

use crate::{analysis::semantic_model::SemanticModel, repl::runtime::PetriNet};

pub struct Repl<'n> {
    net: PetriNet<'n>,
}

impl<'n> Repl<'n> {
    pub fn new(model: &'n SemanticModel) -> Self {
        Self {
            net: PetriNet::from_semantic_model(model),
        }
    }

    pub fn run(&mut self) -> ! {
        let mut trace = Vec::new();
        println!("Welcome to the Petri REPL, enter .help to get a list of available commands.");
        loop {
            io::stdout().write("> ".as_bytes()).expect("");
            io::stdout().flush();
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("");

            match buffer.as_str().trim() {
                ".quit" => process::exit(0),
                ".help" => {
                    print!(
                        ".quit: Exit the repl and program\n.help: Prints help\ntrace: Prints trace of fired transitions\nfire <transition name>: fires a transition\nshow: Shows the state of the net\n"
                    )
                }
                "show" => self.net.show_state(),
                "trace" => {
                    println!("{}", trace.join("->"))
                }
                cmd if cmd.starts_with("fire ") => {
                    let transition_name = cmd.trim_start_matches("fire ").trim();
                    let transition_idx = self
                        .net
                        .transitions()
                        .iter()
                        .position(|transition| transition.name == transition_name);

                    if let Some(transition_idx) = transition_idx {
                        self.net.fire(transition_idx);
                        trace.push(self.net.transitions()[transition_idx].name.clone());
                    }
                }
                _ => println!("Unknown command. Type .help to get a detailed overview."),
            }
        }
    }
}
