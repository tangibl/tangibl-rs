use std::f64::consts::PI;

use topcodes::TopCode;

use crate::{
    ast::{Flow, Start},
    Token, TokenCode,
};

// TODO: Commit some images from my honours explaining these measurements and how they relate to
// each other, with a link to the image in source.

/// An arbitrarily chosen value such that all the following values are in ratio. The original
/// Tangibl tokens were designed in pixels with these measurements.
const TOKEN_SIZE: f64 = 100.0;
/// The radius of the actual TopCode circle.
const TOPCODE_RADIUS: f64 = 24.0;
/// The diameter of the actual TopCode circle.
const TOPCODE_DIAMETER: f64 = TOPCODE_RADIUS * 2.0;
/// The TopCode horizontal offset.
const TOPCODE_CENTER_X: f64 = 50.0;
/// The TopCode vertical offset. It is actually at 62, but counting from the other side is convenient for the trig.
const TOPCODE_CENTER_Y: f64 = 38.0;
/// The point at which the 'true' and 'false' paths of conditional Tokens are closest in distance
/// while still maintaining the same angle.
const CONDITIONAL_INTERSECTION: f64 = 65.0;
/// The maximum distance for a Token to be considered within an acceptable distance of the previous
/// token.
const DISTANCE_TOLERANCE: f64 = TOPCODE_RADIUS * TOPCODE_RADIUS;
/// The maximum angle deviation, in radians, from the expected angle of the previous token.
const ANGLE_TOLERANCE: f64 = PI / 5.0;

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

    pub fn parse(&self) -> Option<Start> {
        let start_token = self
            .tokens
            .iter()
            .find(|token| token.code == TokenCode::Start);

        return self.parse_start(start_token);
    }

    fn parse_start(&self, start_token: Option<&Token>) -> Option<Start> {
        start_token.map(|token| {
            let next_token = self.find_adjacent_token(token);
            Start {
                next: self.parse_flow(next_token),
            }
        })
    }

    fn parse_flow(&self, current_token: Option<&Token>) -> Option<Flow> {
        // TODO
        None
    }

    fn find_adjacent_token(&self, token: &Token) -> Option<&Token> {
        // TODO
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_parse_a_start_token() {
        let parser = Parser::new(&vec![TopCode::new(TokenCode::Start.value())]);
        let result = parser.parse();
        assert_eq!(Some(Start { next: None }), result);
    }
}
