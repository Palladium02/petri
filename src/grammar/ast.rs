use std::range::Range;

pub type Program = Vec<Statement>;

pub enum Statement {
    Place {
        location: Range<usize>,
        name: String,
        tokens: usize,
        label: Option<String>,
    },
    Transition {
        location: Range<usize>,
        name: String,
        label: Option<String>,
    },
    Arc {
        location: Range<usize>,
        pairs: Vec<(String, String, usize)>,
    },
}
