use topcodes::TopCode;

use crate::Token;

pub(crate) struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(topcodes: &Vec<TopCode>) -> Self {
        let mut tokens = Vec::with_capacity(topcodes.len());

        for topcode in topcodes {}

        Self { tokens }
    }

    pub fn parse(&mut self) {}
}
