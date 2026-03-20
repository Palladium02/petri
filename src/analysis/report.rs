use std::{fmt, range::Range};

use crate::traits::Outline;

#[derive(Debug)]
pub enum Severity {
    Error,
    Warning,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::new();

        match self {
            Self::Error => {
                string.push_str("\x1b[31m");
                string.push_str("Error");
            }
            Self::Warning => {
                string.push_str("\x1b[33m");
                string.push_str("Warning");
            }
        }

        string.push_str("\x1b[0m");
        string.push(':');
        write!(f, "{string}")
    }
}

#[derive(Debug)]
pub enum Reason {
    UnusedSymbol,
    UndefinedSymbol,
    RedeclarationOfSymbol,
    BipartideViolation,
    RedeclarationOfArc,
    ZeroAsWeight,
}

#[derive(Debug)]
pub struct Diagnostic {
    severity: Severity,
    primary: Range<usize>,
    related: Vec<Range<usize>>,
    reason: Reason,
}

impl Diagnostic {
    pub fn to_string(&self, source: &str) -> String {
        let severity = self.severity.to_string();

        match self.reason {
            Reason::UnusedSymbol => {
                let outline = source.outline(self.primary);
                format!(
                    "{severity} Found unused but declared symbol: \n{outline}\nConsider removing the declaration, if this is intentional you can ignore this warning."
                )
            }
            Reason::UndefinedSymbol => {
                let outline = source.outline(self.primary);
                format!("{severity} Use of undeclared symbol: \n{outline}")
            }
            Reason::RedeclarationOfSymbol => {
                let primary_outline = source.outline(self.primary);
                let secondary_outline = source.outline(self.related[0]);
                format!(
                    "{severity} Redeclaration of already existing symbol.\nFirst found here:\n{primary_outline}\n\nLater redeclared here:\n{secondary_outline}\n\nConsider removing or renaming one of the symbols."
                )
            }
            Reason::BipartideViolation => {
                let outline = source.outline(self.primary);
                format!(
                    "{severity} Violation of the bipartide rule found. Arc must be either Place->Transition or Transition->Place:\nViolation found in:\n{outline}\nNote: This error can also be cause by two subsequent undefined symbols."
                )
            }
            Reason::RedeclarationOfArc => {
                let primary_outline = source.outline(self.primary);
                let secondary_outline = source.outline(self.related[0]);
                format!(
                    "{severity} Redeclaration of already existing arc.\nFirst found here:\n{primary_outline}\n\nLater redeclared here:\n{secondary_outline}\n\nConsider removing one declaration. If this was intentional try merging the arc declarations."
                )
            }
            Reason::ZeroAsWeight => {
                let outline = source.outline(self.primary);
                format!(
                    "{severity} Found arc weight that has been declared as 0, weights must be >= 1:\n{outline}"
                )
            }
        }
    }
}

#[derive(Debug)]
pub struct Report {
    diagnostics: Vec<Diagnostic>,
}

impl Report {
    pub const fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn error(&mut self, span: Range<usize>, reason: Reason) {
        self.diagnostics.push(Diagnostic {
            severity: Severity::Error,
            primary: span,
            related: Vec::new(),
            reason,
        });
    }

    pub fn error_with_related(
        &mut self,
        primary: Range<usize>,
        related: Vec<Range<usize>>,
        reason: Reason,
    ) {
        self.diagnostics.push(Diagnostic {
            severity: Severity::Error,
            primary,
            related,
            reason,
        });
    }

    pub fn warning(&mut self, span: Range<usize>, reason: Reason) {
        self.diagnostics.push(Diagnostic {
            severity: Severity::Warning,
            primary: span,
            related: Vec::new(),
            reason,
        });
    }

    pub fn has_error(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| matches!(diagnostic.severity, Severity::Error))
    }

    pub fn to_string(&self, source: &str) -> String {
        self.diagnostics
            .iter()
            .map(|diagnostic| diagnostic.to_string(source))
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}
