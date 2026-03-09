use std::range::Range;

use crate::grammar::lexer::Token;

pub trait Extract {
    type Output;
    fn extract(token: Token) -> Option<Self::Output>;
}

pub trait RangeExt<T> {
    fn merge(&self, other: &Range<T>) -> Range<T>;
}

impl RangeExt<usize> for Range<usize> {
    fn merge(&self, other: &Range<usize>) -> Range<usize> {
        Range::from(self.start.min(other.start)..self.end.max(other.end))
    }
}
