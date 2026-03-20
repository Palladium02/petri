use std::{collections::HashMap, range::Range};

use crate::{
    analysis::report::{Reason, Report},
    grammar::ast::{Program, Statement},
};

pub enum Symbol {
    Place { name: String, span: Range<usize> },
    Transition { name: String, span: Range<usize> },
    Error,
}

impl Symbol {
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Place { name, span } => Some(name),
            Self::Transition { name, span } => Some(name),
            Self::Error => None,
        }
    }

    pub const fn span(&self) -> Option<&Range<usize>> {
        match self {
            Self::Place { name, span } => Some(span),
            Self::Transition { name, span } => Some(span),
            Self::Error => None,
        }
    }

    pub const fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}

pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    pub const fn table(&self) -> &HashMap<String, Symbol> {
        &self.symbols
    }

    pub fn from_ast(ast: &Program, report: &mut Report) -> Self {
        let symbol_declarations = ast.iter().filter_map(|statement| match statement {
            Statement::Place { location, name, .. } => Some(Symbol::Place {
                name: name.to_owned(),
                span: *location,
            }),
            Statement::Transition { location, name, .. } => Some(Symbol::Transition {
                name: name.to_owned(),
                span: *location,
            }),
            Statement::Arc { .. } => None,
        });

        let mut symbols: HashMap<String, Symbol> = HashMap::new();

        for symbol in symbol_declarations {
            if let Some(existing_symbol) = symbols.get(symbol.name().expect("We can safely unwrap here because we have never constructed error symbols prior to this call.")) {
                report.error_with_related(*symbol.span().expect("Same reason as above."), vec![*existing_symbol.span().expect("Same reason as above.")], Reason::RedeclarationOfSymbol);
            } else {
                symbols.insert(symbol.name().expect("Same reason as above.").to_owned(), symbol);
            }
        }

        Self { symbols }
    }
}
