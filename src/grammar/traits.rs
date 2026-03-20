use std::range::Range;

use crate::grammar::{
    ast::{Program, Statement},
    lexer::Token,
};

pub trait Extract {
    type Output;
    fn extract(token: Token) -> Option<Self::Output>;
}

pub trait RangeExt {
    fn merge(&self, other: &Self) -> Self;
}

impl RangeExt for Range<usize> {
    fn merge(&self, other: &Self) -> Self {
        Self::from(self.start.min(other.start)..self.end.max(other.end))
    }
}
