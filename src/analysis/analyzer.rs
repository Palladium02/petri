use std::collections::{HashMap, HashSet};

use crate::{
    analysis::{
        report::{Reason, Report},
        semantic_model::SemanticModel,
        symbol::{Symbol, SymbolTable},
    },
    grammar::ast::Program,
};

pub struct Analyzer;

impl Analyzer {
    pub fn analyze(ast: &Program) -> (SemanticModel, Report) {
        let mut report = Report::new();

        let symbols = SymbolTable::from_ast(ast, &mut report);
        let model = SemanticModel::from_ast(ast, symbols);

        Self::check_rules(&model, &mut report);

        (model, report)
    }

    fn check_rules(model: &SemanticModel, report: &mut Report) {
        let used_symbols = Self::check_arcs(model, report);
        Self::check_unused(model, &used_symbols, report);
    }

    fn check_arcs(model: &SemanticModel, report: &mut Report) -> HashSet<String> {
        let mut used_symbols = HashSet::new();
        let mut used_pairs = HashMap::new();

        for arc in model.arcs() {
            let left = model
                .symbols()
                .get(arc.left())
                .inspect(|symbol| {
                    used_symbols.insert(
                        symbol
                            .name()
                            .expect("We never insert error symbols into the symbol table")
                            .to_owned(),
                    );
                })
                .unwrap_or(&Symbol::Error);

            let right = model
                .symbols()
                .get(arc.right())
                .inspect(|symbol| {
                    used_symbols.insert(
                        symbol
                            .name()
                            .expect("We never insert error symbols into the symbol table")
                            .to_owned(),
                    );
                })
                .unwrap_or(&Symbol::Error);

            match (left, right) {
                (Symbol::Place { .. }, Symbol::Place { .. })
                | (Symbol::Transition { .. }, Symbol::Transition { .. }) => {
                    report.error(*arc.span(), Reason::BipartideViolation);
                }
                _ => {}
            }

            if *arc.weight() == 0 {
                report.error(*arc.span(), Reason::ZeroAsWeight);
            }

            if !(left.is_error() && right.is_error()) {
                let key = (left.name().unwrap(), right.name().unwrap());
                if let Some(span) = used_pairs.get(&key) {
                    report.error_with_related(*arc.span(), vec![*span], Reason::RedeclarationOfArc);
                } else {
                    used_pairs.insert(key, *arc.span());
                }
            }
        }
        used_symbols
    }

    fn check_unused(model: &SemanticModel, used_symbols: &HashSet<String>, report: &mut Report) {
        for (name, symbol) in model.symbols().table() {
            if !used_symbols.contains(name) {
                report.warning(*symbol.span().unwrap(), Reason::UnusedSymbol);
            }
        }
    }
}
