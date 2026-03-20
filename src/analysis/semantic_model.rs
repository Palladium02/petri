use std::{collections::HashMap, range::Range};

use crate::{
    analysis::{
        report::{Reason, Report},
        symbol::SymbolTable,
    },
    grammar::ast::{Program, Statement},
};

pub struct Place {
    name: String,
    tokens: usize,
    label: Option<String>,
}

impl Place {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn tokens(&self) -> &usize {
        &self.tokens
    }

    pub const fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }
}

pub struct Transition {
    name: String,
    label: Option<String>,
}

impl Transition {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }
}

pub struct Arc {
    left: String,
    right: String,
    weight: usize,
    span: Range<usize>,
}

impl Arc {
    pub fn left(&self) -> &str {
        &self.left
    }

    pub fn right(&self) -> &str {
        &self.right
    }

    pub const fn span(&self) -> &Range<usize> {
        &self.span
    }

    pub const fn weight(&self) -> &usize {
        &self.weight
    }
}

pub struct SemanticModel {
    symbols: SymbolTable,
    places: Vec<Place>,
    transitions: Vec<Transition>,
    arcs: Vec<Arc>,
}

impl SemanticModel {
    pub fn from_ast(ast: &Program, symbols: SymbolTable) -> Self {
        Self {
            symbols,
            places: Self::extract_places(ast),
            transitions: Self::extract_transitions(ast),
            arcs: Self::extract_arcs(ast),
        }
    }

    pub const fn symbols(&self) -> &SymbolTable {
        &self.symbols
    }

    pub const fn places(&self) -> &Vec<Place> {
        &self.places
    }

    pub const fn transitions(&self) -> &Vec<Transition> {
        &self.transitions
    }

    pub const fn arcs(&self) -> &Vec<Arc> {
        &self.arcs
    }

    fn extract_places(ast: &Program) -> Vec<Place> {
        ast.iter()
            .filter_map(|statement| match statement {
                Statement::Place {
                    name,
                    tokens,
                    label,
                    ..
                } => Some(Place {
                    name: name.to_owned(),
                    tokens: *tokens,
                    label: label.clone(),
                }),
                _ => None,
            })
            .collect()
    }

    fn extract_transitions(ast: &Program) -> Vec<Transition> {
        ast.iter()
            .filter_map(|statement| match statement {
                Statement::Transition { name, label, .. } => Some(Transition {
                    name: name.clone(),
                    label: label.clone(),
                }),
                _ => None,
            })
            .collect()
    }

    fn extract_arcs(ast: &Program) -> Vec<Arc> {
        ast.iter()
            .filter_map(|statement| match statement {
                Statement::Arc { location, pairs } => Some(
                    pairs
                        .iter()
                        .map(|triplet| Arc {
                            left: triplet.0.clone(),
                            right: triplet.1.clone(),
                            weight: triplet.2,
                            span: *location,
                        })
                        .collect::<Vec<Arc>>(),
                ),
                _ => None,
            })
            .flatten()
            .collect()
    }
}
