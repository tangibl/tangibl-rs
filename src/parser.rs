use std::f64::consts::PI;

use topcodes::TopCode;

use crate::{
    ast::{Command, Flow, FlowKind, Start},
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
const TWO_PI: f64 = PI * 2.0;

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
            let next_token = self.find_adjacent_token(token, None);
            Start {
                next: self.parse_flow(next_token),
            }
        })
    }

    fn parse_flow(&self, current_token: Option<&Token>) -> Option<Flow> {
        current_token.map(|token| match token.code {
            TokenCode::Shoot
            | TokenCode::TurnLeft
            | TokenCode::TurnRight
            | TokenCode::MoveForwards
            | TokenCode::MoveBackwards => self.parse_command(token),
            _ => todo!(),
        })
    }

    fn parse_command(&self, current_token: &Token) -> Flow {
        let command = match current_token.code {
            TokenCode::Shoot => Command::Shoot,
            _ => panic!("Received a non-command token during parse_command"),
        };
        Flow {
            kind: FlowKind::Command(command),
            next: self
                .parse_flow(self.find_adjacent_token(current_token, None))
                .map(|flow| Box::new(flow)),
        }
    }

    /// Given a flow token, find the token which is adjacent to it.
    fn find_adjacent_token(&self, token: &Token, parent: Option<&Token>) -> Option<&Token> {
        let ratio = token.diameter / TOPCODE_DIAMETER;
        let distance = ratio * TOKEN_SIZE;
        let x = token.x + (distance * token.orientation.cos());
        let y = token.y + (distance * token.orientation.sin());

        for candidate in &self.tokens {
            if candidate != token && candidate.is_flow() {
                if let Some(parent) = parent {
                    if parent == candidate {
                        continue;
                    }
                }

                let delta_displacement_squared =
                    (candidate.x - x).powi(2) + (candidate.y - y).powi(2);
                let mut delta_angle = (candidate.orientation - token.orientation) % TWO_PI;
                delta_angle = f64::min(delta_angle, TWO_PI - delta_angle);
                if delta_displacement_squared < DISTANCE_TOLERANCE && delta_angle < ANGLE_TOLERANCE
                {
                    return Some(candidate);
                }
            }
        }

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

    #[test]
    fn it_can_parse_a_trivial_flow() {
        let parser = Parser::new(&vec![
            TopCode::mock(TokenCode::Start.value(), 6.0, 0.0, 0.0, 0.0),
            TopCode::mock(TokenCode::Shoot.value(), 6.0, 0.0, 100.0, 0.0),
        ]);
        let result = parser.parse();
        assert_eq!(
            Some(Start {
                next: Some(Flow {
                    next: None,
                    kind: FlowKind::Command(Command::Shoot)
                })
            }),
            result
        );
    }
}
