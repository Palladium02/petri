use crate::grammar::{lexer::Token, traits::Extract};

macro_rules! token_extract {
    ($name:ident, $output:ty, $pattern:pat => $value:expr) => {
        pub struct $name;

        impl Extract for $name {
            type Output = $output;

            fn extract(token: Token) -> Option<Self::Output> {
                match token {
                    $pattern => Some($value),
                    _ => None,
                }
            }
        }
    };
}

token_extract!(Place, (), Token::Place => ());
token_extract!(Transition, (), Token::Transition => ());
token_extract!(Tokens, (), Token::Tokens => ());
token_extract!(Identifier, String, Token::Identifier(identifier) => identifier);
token_extract!(IntegerLiteral, usize, Token::Integer(integer) => integer);
token_extract!(StringLiteral, String, Token::String(string) => string);
token_extract!(Arrow, (), Token::Arrow => ());
token_extract!(LeftBracket, (), Token::LeftBracket => ());
token_extract!(RightBracket, (), Token::RightBracket => ());
token_extract!(Semicolon, (), Token::Semicolon => ());
token_extract!(Equals, (), Token::Equals => ());
