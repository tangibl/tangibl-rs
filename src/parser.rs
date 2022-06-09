use topcodes::TopCode;

use crate::{Token, TokenCode};

pub(crate) struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(topcodes: &Vec<TopCode>) -> Self {
        let mut tokens = Vec::with_capacity(topcodes.len());

        for topcode in topcodes {
            if let Some(code) = topcode.code {
                if let Ok(token_code) = TokenCode::try_from(code) {
                    let token = Token::new(
                        token_code,
                        topcode.unit * 8.0,
                        topcode.orientation,
                        topcode.x,
                        topcode.y,
                    );
                    tokens.push(token)
                }
            }
        }

        Self { tokens }
    }

    pub fn parse(&mut self) {}
}
