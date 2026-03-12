use std::{
    collections::{HashMap, HashSet, hash_map::Entry},
    marker::PhantomData,
    range::Range,
};

use crate::grammar::ast::{Program, Statement};

#[derive(Debug, Clone)]
pub struct Report {
    diagnostics: Vec<Diagnostic>,
}

impl Report {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    severity: Severity,
    span: Range<usize>,
    reason: Reason,
}

#[derive(Debug, Clone)]
pub enum Reason {
    Redeclaration {
        first: Symbol,
        second: Symbol,
    },
    UndefinedSymbol {
        symbol: String,
        location: Range<usize>,
    },
    BipartideViolation {
        location: Range<usize>,
        left: Symbol,
        right: Symbol,
    },
    UnusedSymbol {
        symbol: String,
    },
    DuplicateArc {
        first: Range<usize>,
        second: Range<usize>,
    },
    ZeroWeight,
}

#[derive(Debug, Clone)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub enum Symbol {
    Place {
        name: String,
        label: Option<String>,
        tokens: usize,
        location: Range<usize>,
    },
    Transition {
        name: String,
        label: Option<String>,
        location: Range<usize>,
    },
    Error,
}

impl Symbol {
    pub fn name(&self) -> &str {
        match self {
            Self::Place {
                name,
                label,
                tokens,
                location,
            } => name,
            Self::Transition {
                name,
                label,
                location,
            } => name,
            Self::Error => "",
        }
    }

    pub fn location(&self) -> Range<usize> {
        match self {
            Self::Place {
                name,
                label,
                tokens,
                location,
            } => *location,
            Self::Transition {
                name,
                label,
                location,
            } => *location,
            Self::Error => Range::from(0..0),
        }
    }
}

pub struct Analyzer {
    report: Report,
    ast: Program,
}

impl Analyzer {
    pub fn new(ast: Program) -> Self {
        Self {
            report: Report::new(),
            ast,
        }
    }

    pub fn analyze(&mut self) -> Report {
        let symbol_table = self.symbol_table();
        let used_symbols = self.check_bipartide(&symbol_table);

        self.report.clone()
    }

    fn symbol_table(&mut self) -> HashMap<String, Symbol> {
        let symbol_declarations = self
            .ast
            .iter()
            .filter_map(|statement| match statement {
                Statement::Place {
                    location,
                    name,
                    tokens,
                    label,
                } => Some(Symbol::Place {
                    name: name.to_owned(),
                    label: label.clone(),
                    tokens: *tokens,
                    location: *location,
                }),
                Statement::Transition {
                    location,
                    name,
                    label,
                } => Some(Symbol::Transition {
                    name: name.to_owned(),
                    label: label.clone(),
                    location: *location,
                }),
                _ => None,
            })
            .collect::<Vec<Symbol>>();

        let mut symbol_table: HashMap<String, Symbol> = HashMap::new();

        for symbol in symbol_declarations {
            let symbol_entry = symbol_table.entry(symbol.name().to_owned());
            match symbol_entry {
                Entry::Occupied(occupied_entry) => {
                    let existing_symbol = occupied_entry.get();
                    self.report.diagnostics.push(Diagnostic {
                        severity: Severity::Error,
                        span: symbol.location(),
                        reason: Reason::Redeclaration {
                            first: existing_symbol.clone(),
                            second: symbol.clone(),
                        },
                    });
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(symbol);
                }
            }
        }

        symbol_table
    }

    fn check_bipartide(&mut self, symbol_table: &HashMap<String, Symbol>) -> HashSet<String> {
        use Symbol::*;

        let mut used_symbols = HashSet::new();
        let mut used_pairs = HashMap::new();
        let mut diagnostics = Vec::new();

        for stmt in &self.ast {
            if let Statement::Arc { location, pairs } = stmt {
                for (left_name, right_name, weight) in pairs {
                    let left = match symbol_table.get(left_name) {
                        Some(sym) => {
                            used_symbols.insert(left_name.clone());
                            sym
                        }
                        None => {
                            diagnostics.push(Diagnostic {
                                severity: Severity::Error,
                                span: *location,
                                reason: Reason::UndefinedSymbol {
                                    symbol: left_name.clone(),
                                    location: *location,
                                },
                            });
                            &Symbol::Error
                        }
                    };

                    let right = match symbol_table.get(right_name) {
                        Some(sym) => {
                            used_symbols.insert(right_name.clone());
                            sym
                        }
                        None => {
                            diagnostics.push(Diagnostic {
                                severity: Severity::Error,
                                span: *location,
                                reason: Reason::UndefinedSymbol {
                                    symbol: right_name.clone(),
                                    location: *location,
                                },
                            });
                            &Symbol::Error
                        }
                    };

                    match (left, right) {
                        (Place { .. }, Place { .. }) | (Transition { .. }, Transition { .. }) => {
                            diagnostics.push(Diagnostic {
                                severity: Severity::Error,
                                span: *location,
                                reason: Reason::BipartideViolation {
                                    location: *location,
                                    left: left.clone(),
                                    right: right.clone(),
                                },
                            });
                        }
                        _ => {}
                    }

                    if *weight == 0 {
                        diagnostics.push(Diagnostic {
                            severity: Severity::Error,
                            span: *location,
                            reason: Reason::ZeroWeight,
                        });
                    }

                    if let Some(arc_location) = used_pairs.get(&(left.name(), right.name())) {
                        diagnostics.push(Diagnostic {
                            severity: Severity::Error,
                            span: *location,
                            reason: Reason::DuplicateArc {
                                first: *arc_location,
                                second: *location,
                            },
                        });
                    } else {
                        used_pairs.insert((left.name(), right.name()), *location);
                    }
                }
            }
        }
        self.report.diagnostics.extend(diagnostics);
        used_symbols
    }

    fn check_unused(
        &mut self,
        symbol_table: &HashMap<String, Symbol>,
        used_symbols: &HashSet<String>,
    ) {
        for (name, symbol) in symbol_table {
            if !used_symbols.contains(name) {
                self.report.diagnostics.push(Diagnostic {
                    severity: Severity::Warning,
                    span: symbol.location(),
                    reason: Reason::UnusedSymbol {
                        symbol: name.clone(),
                    },
                });
            }
        }
    }
}
