use std::collections::HashMap;

use crate::analysis::{semantic_model::SemanticModel, symbol::Symbol};

pub struct Place {
    pub name: String,
    tokens: usize,
}

pub struct Transition {
    pub name: String,
    inputs: Vec<(usize, usize)>,
    outputs: Vec<(usize, usize)>,
}

impl Transition {
    pub fn is_enabled(&self, net: &PetriNet) -> bool {
        self.inputs
            .iter()
            .all(|(p, count)| net.places[*p].tokens >= *count)
    }
}

pub struct PetriNet<'k> {
    places: Vec<Place>,
    transitions: Vec<Transition>,
    index: HashMap<&'k str, usize>,
}

impl<'k> PetriNet<'k> {
    pub fn transitions(&self) -> &Vec<Transition> {
        &self.transitions
    }

    pub fn index(&self) -> &HashMap<&str, usize> {
        &self.index
    }

    pub fn is_enabled(&self, transition_idx: usize) -> bool {
        let transition = &self.transitions[transition_idx];
        transition
            .inputs
            .iter()
            .all(|(place_idx, count)| self.places[*place_idx].tokens >= *count)
    }

    pub fn fire(&mut self, transition_idx: usize) {
        if !self.is_enabled(transition_idx) {
            println!(
                "Transition {} is not enabled",
                self.transitions[transition_idx].name
            );
            return;
        }

        let transition = &self.transitions[transition_idx];

        for (place_idx, count) in &transition.inputs {
            self.places[*place_idx].tokens -= *count;
        }

        for (p, count) in &transition.outputs {
            self.places[*p].tokens += *count;
        }

        println!("Fired transition {}", transition.name);
    }

    pub fn enabled_transitions(&self) -> Vec<&Transition> {
        self.transitions
            .iter()
            .filter(|transition| transition.is_enabled(self))
            .collect()
    }

    pub fn show_state(&self) {
        println!("Places:");
        for place in &self.places {
            println!("  {}: {} token(s)", place.name, place.tokens);
        }

        println!("Enabled transitions:");
        for transition in self.enabled_transitions() {
            println!("  {}", transition.name);
        }
    }

    pub fn from_semantic_model(model: &'k SemanticModel) -> Self {
        let mut index = HashMap::new();
        let mut places = Vec::new();
        let mut transitions = Vec::new();

        model.places().iter().enumerate().for_each(|(idx, place)| {
            places.push(Place {
                name: place.name().to_owned(),
                tokens: *place.tokens(),
            });

            index.insert(place.name(), idx);
        });

        model
            .transitions()
            .iter()
            .enumerate()
            .for_each(|(idx, transition)| {
                transitions.push(Transition {
                    name: transition.name().to_owned(),
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                });

                index.insert(transition.name(), idx);
            });

        model.arcs().iter().for_each(|arc| {
            let symbols = model.symbols();
            let lr = (
                symbols.get(arc.left()).unwrap(),
                symbols.get(arc.right()).unwrap(),
            );

            match lr {
                (place @ Symbol::Place { .. }, transition @ Symbol::Transition { .. }) => {
                    let transition_idx = index.get(transition.name().unwrap()).unwrap();
                    let place_idx = index.get(place.name().unwrap()).unwrap();
                    let mut transition = transitions.get_mut(*transition_idx).unwrap();
                    transition.inputs.push((*place_idx, *arc.weight()));
                }
                (transition @ Symbol::Transition { .. }, place @ Symbol::Place { .. }) => {
                    let transition_idx = index.get(transition.name().unwrap()).unwrap();
                    let place_idx = index.get(place.name().unwrap()).unwrap();
                    let mut transition = transitions.get_mut(*transition_idx).unwrap();
                    transition.outputs.push((*place_idx, *arc.weight()));
                }
                _ => unreachable!(),
            }
        });

        Self {
            places,
            transitions,
            index,
        }
    }
}
