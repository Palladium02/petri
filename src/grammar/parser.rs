use std::{iter::Peekable, range::Range};

use crate::grammar::{
    ast::{Program, Statement},
    extract::{
        Arrow, Equals, Identifier, IntegerLiteral, LeftBracket, Place, RightBracket, Semicolon,
        StringLiteral, Tokens, Transition,
    },
    lexer::{Lexer, SpannedToken, Token, TokenKind},
    traits::{Extract, RangeExt},
};

pub enum ParseError {
    UnexpectedEoF,
    UnexpectedToken(SpannedToken),
}

pub struct Parser<'t> {
    input: Peekable<Lexer<'t>>,
}

impl<'t> Parser<'t> {
    pub fn new(input: Lexer<'t>) -> Self {
        Self {
            input: input.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();

        while !self.is_eof() {
            statements.push(self.expect_statement()?);
        }

        Ok(statements)
    }

    pub fn expect_statement(&mut self) -> Result<Statement, ParseError> {
        let Some((token, span)) = self.input.peek() else {
            return Err(ParseError::UnexpectedEoF);
        };

        match token {
            Token::Place => self.expect_place_declaration(),
            Token::Transition => self.expect_transition_declaration(),
            Token::Identifier(_) => self.expect_arc_declaration(),
            _ => Err(ParseError::UnexpectedToken((token.clone(), *span))),
        }
    }

    pub fn expect_place_declaration(&mut self) -> Result<Statement, ParseError> {
        let (_, start) = self.expect::<Place>()?;

        let (name, _) = self.expect::<Identifier>()?;

        let label = if self.peek::<StringLiteral>() {
            Some(self.expect::<StringLiteral>()?.0)
        } else {
            None
        };

        let tokens = if self.peek::<Tokens>() {
            let _ = self.expect::<Tokens>()?;
            let _ = self.expect::<Equals>()?;
            self.expect::<IntegerLiteral>()?.0
        } else {
            0
        };

        let (_, end) = self.expect::<Semicolon>()?;

        Ok(Statement::Place {
            location: start.merge(&end),
            name,
            tokens,
            label,
        })
    }

    pub fn expect_transition_declaration(&mut self) -> Result<Statement, ParseError> {
        let (_, start) = self.expect::<Transition>()?;

        let (name, _) = self.expect::<Identifier>()?;

        let label = if self.peek::<StringLiteral>() {
            Some(self.expect::<StringLiteral>()?.0)
        } else {
            None
        };

        let (_, end) = self.expect::<Semicolon>()?;

        Ok(Statement::Transition {
            location: start.merge(&end),
            name,
            label,
        })
    }

    pub fn expect_arc_declaration(&mut self) -> Result<Statement, ParseError> {
        enum Value {
            String(String),
            Int(usize),
        }

        let mut nodes = Vec::new();

        let (fst, start) = self.expect::<Identifier>()?;

        let weight = self.expect_weighted_arrow()?;
        let (snd, _) = self.expect::<Identifier>()?;

        nodes.push(Value::String(snd));
        nodes.push(Value::Int(weight));

        while !self.peek::<Semicolon>() {
            let weight = self.expect_weighted_arrow()?;
            let (nd, _) = self.expect::<Identifier>()?;

            nodes.push(Value::String(nd));
            nodes.push(Value::Int(weight));
        }

        let (_, end) = self.expect::<Semicolon>()?;

        let pairs = nodes
            .windows(3)
            .step_by(2)
            .map(|window| match window {
                [
                    Value::String(left),
                    Value::Int(weight),
                    Value::String(right),
                ] => (left.clone(), right.clone(), *weight),
                _ => unreachable!(),
            })
            .collect::<Vec<(String, String, usize)>>();

        Ok(Statement::Arc {
            location: start.merge(&end),
            pairs,
        })
    }

    pub fn expect_weighted_arrow(&mut self) -> Result<usize, ParseError> {
        self.expect::<Arrow>()?;
        let weight = if self.peek::<LeftBracket>() {
            self.expect::<LeftBracket>()?;
            let (weight, _) = self.expect::<IntegerLiteral>()?;
            self.expect::<RightBracket>()?;

            weight
        } else {
            1
        };

        Ok(weight)
    }

    pub fn expect<E: Extract>(&mut self) -> Result<(E::Output, Range<usize>), ParseError> {
        match self.input.next() {
            Some((token, span)) => match E::extract(token.clone()) {
                Some(value) => Ok((value, span)),
                None => Err(ParseError::UnexpectedToken((token, span))),
            },
            None => Err(ParseError::UnexpectedEoF),
        }
    }

    pub fn peek<E: Extract>(&mut self) -> bool {
        self.input
            .peek()
            .and_then(|(token, _)| E::extract(token.clone()))
            .is_some()
    }

    pub fn is_eof(&mut self) -> bool {
        self.input.peek().is_some()
    }
}
