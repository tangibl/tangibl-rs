use std::f64::{self, consts::PI};

use lazy_static::lazy_static;
use topcodes::TopCode;

use crate::{
    ast::{
        BooleanMethod, BooleanMethodKind, Command, Condition, Conditional, ConditionalKind, Flow,
        FlowKind, IntegerMethod, IntegerMethodKind, Start, Value,
    },
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
/// The maximum displacement squared for a Token to be considered within an acceptable range of the previous
/// token.
const DISPLACEMENT_SQUARED_TOLERANCE: f64 = (TOPCODE_RADIUS * 2.5) * (TOPCODE_RADIUS * 2.5);
/// The maximum angle deviation, in radians, from the expected angle of the previous token.
const ANGLE_TOLERANCE: f64 = PI / 5.0;
const TWO_PI: f64 = PI * 2.0;

// The following are pre-calculated helper values for working with the conditional token, as it has
// a more complicated form-factor compared to the other Tangibl tokens.
/// The point at which the 'true' and 'false' paths of conditional Tokens are closest in distance
/// while still maintaining the same angle.
const CONDITIONAL_INTERSECTION: f64 = 65.0;
// Additionally, consts cannot be used until the following issue is resolved in the Rust standard
// library (Since floats behave differently on many platforms):
//
// https://github.com/rust-Lang/rust/issues/57241
lazy_static! {
    static ref TRUE_HYPOTENUSE: f64 = (TOPCODE_CENTER_X.powi(2) + TOPCODE_CENTER_Y.powi(2)).sqrt();
    static ref TRUE_TOKEN_X: f64 = *TRUE_HYPOTENUSE
        * ((PI / 4.0) - (TOPCODE_CENTER_Y / *TRUE_HYPOTENUSE).asin()).cos()
        + CONDITIONAL_INTERSECTION
        - TOPCODE_CENTER_X;
    static ref TRUE_TOKEN_Y: f64 = *TRUE_HYPOTENUSE
        * ((PI / 4.0) - (TOPCODE_CENTER_Y / *TRUE_HYPOTENUSE).asin()).sin()
        + TOPCODE_CENTER_Y;
    static ref FALSE_HYPOTENUSE: f64 =
        (TOPCODE_CENTER_X.powi(2) + (TOKEN_SIZE - TOPCODE_CENTER_Y).powi(2)).sqrt();
    static ref FALSE_TOKEN_X: f64 = *FALSE_HYPOTENUSE
        * ((PI / 4.0) - (TOPCODE_CENTER_X / *FALSE_HYPOTENUSE).asin()).cos()
        + CONDITIONAL_INTERSECTION
        - TOPCODE_CENTER_X;
    static ref FALSE_TOKEN_Y: f64 = *FALSE_HYPOTENUSE
        * ((PI / 4.0) - (TOPCODE_CENTER_X / *FALSE_HYPOTENUSE).asin()).sin()
        - (TOKEN_SIZE - TOPCODE_CENTER_Y);
}

pub(crate) struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(topcodes: &Vec<TopCode>) -> Self {
        let mut tokens = Vec::with_capacity(topcodes.len());

        let mut diameter_sum = 0.0;
        for topcode in topcodes {
            if let Some(code) = topcode.code {
                if let Ok(token_code) = TokenCode::try_from(code) {
                    diameter_sum += topcode.unit * 8.0;
                    // Token orientation and y values are flipped so that they follow mathematical
                    // convention instead of image convention. I may change this in future, but it
                    // is only an internal representation which should not affect the contract of
                    // the parser.
                    let token = Token::new(
                        token_code,
                        0.0,
                        Self::get_angle(-topcode.orientation),
                        topcode.x,
                        -topcode.y,
                    );
                    tokens.push(token)
                }
            }
        }

        // Averaging out the diameter across all TopCodes to produce more accurate results.
        //
        // TODO: Remove outliers using a sample variance calculation to improve this value.
        let diameter_avg = diameter_sum / tokens.len() as f64;
        for token in &mut tokens {
            token.diameter = diameter_avg;
        }

        Self { tokens }
    }

    fn get_angle(mut angle: f64) -> f64 {
        while angle > TWO_PI {
            angle -= TWO_PI;
        }
        while angle < 0.0 {
            angle += TWO_PI;
        }
        angle
    }

    pub fn parse(&self) -> Option<Start> {
        log::debug!(
            r#"Starting parser with constants: {{
  DISPLACEMENT_SQUARED_TOLERANCE: {},
  ANGLE_TOLERANCE: {}
}}"#,
            DISPLACEMENT_SQUARED_TOLERANCE,
            ANGLE_TOLERANCE,
        );

        let start_token = self
            .tokens
            .iter()
            .find(|token| token.code == TokenCode::Start);

        self.parse_start(start_token)
    }

    fn parse_start(&self, start_token: Option<&Token>) -> Option<Start> {
        start_token.map(|token| {
            log::debug!("Starting with first start token: {:?}", token);
            let next_token = self.find_adjacent_token(token, None);
            Start {
                next: self.parse_flow(next_token),
            }
        })
    }

    fn parse_flow(&self, current_token: Option<&Token>) -> Option<Flow> {
        log::debug!("Trying to parse flow from token: {:?}", current_token);
        current_token.map(|token| match token.code {
            TokenCode::Shoot
            | TokenCode::TurnLeft
            | TokenCode::TurnRight
            | TokenCode::MoveForwards
            | TokenCode::MoveBackwards => self.parse_command(token),
            TokenCode::Blocked => self.parse_conditional(token),
            TokenCode::While => self.parse_boolean_method(token),
            TokenCode::Repeat => self.parse_integer_method(token),
            _ => panic!("Received a flow token that has not been modelled"),
        })
    }

    fn parse_command(&self, current_token: &Token) -> Flow {
        let command = match current_token.code {
            TokenCode::Shoot => Command::Shoot,
            TokenCode::TurnLeft => Command::TurnLeft,
            TokenCode::TurnRight => Command::TurnRight,
            TokenCode::MoveForwards => Command::MoveForwards,
            TokenCode::MoveBackwards => Command::MoveBackwards,
            _ => panic!("Received a non-command token during parse_command"),
        };
        Flow {
            kind: FlowKind::Command(command),
            next: self
                .parse_flow(self.find_adjacent_token(current_token, None))
                .map(Box::new),
        }
    }

    fn parse_conditional(&self, current_token: &Token) -> Flow {
        let conditional_kind = match current_token.code {
            TokenCode::Blocked => ConditionalKind::Blocked,
            _ => panic!("Received a non-command token during parse_conditional"),
        };
        let true_token = self.find_true_token(current_token);
        let false_token = self.find_false_token(current_token);
        log::debug!(
            "parse_conditional {{ true_token: {:?}, false_token: {:?} }}",
            true_token,
            false_token
        );
        Flow {
            kind: FlowKind::Conditional(Conditional {
                kind: conditional_kind,
                alternate: self.parse_flow(false_token).map(Box::new),
            }),
            next: self.parse_flow(true_token).map(Box::new),
        }
    }

    fn parse_boolean_method(&self, current_token: &Token) -> Flow {
        let boolean_method_kind = match current_token.code {
            TokenCode::While => BooleanMethodKind::While,
            _ => panic!(
                "Received a token which was not a boolean method token in parse_boolean_method"
            ),
        };
        Flow {
            kind: FlowKind::BooleanMethod(BooleanMethod {
                kind: boolean_method_kind,
                body: self
                    .parse_flow(self.find_method_body_token(current_token))
                    .map(Box::new),
                condition: self.parse_condition(
                    self.find_method_parameter_token(current_token, &Token::is_condition),
                ),
            }),
            next: self
                .parse_flow(self.find_adjacent_token(current_token, None))
                .map(Box::new),
        }
    }

    fn parse_condition(&self, candidate: Option<&Token>) -> Option<Condition> {
        candidate.and_then(|token| match token.code {
            TokenCode::IsBlocked => Some(Condition::IsBlocked),
            TokenCode::IsPathClear => Some(Condition::IsPathClear),
            _ => None,
        })
    }

    fn parse_integer_method(&self, current_token: &Token) -> Flow {
        let integer_method_kind = match current_token.code {
            TokenCode::Repeat => IntegerMethodKind::Repeat,
            _ => panic!(
                "Received a token which was not an integer method token in parse_integer_method"
            ),
        };
        Flow {
            kind: FlowKind::IntegerMethod(IntegerMethod {
                kind: integer_method_kind,
                body: self
                    .parse_flow(self.find_method_body_token(current_token))
                    .map(Box::new),
                value: self
                    .parse_value(self.find_method_parameter_token(current_token, &Token::is_value)),
            }),
            next: self
                .parse_flow(self.find_adjacent_token(current_token, None))
                .map(Box::new),
        }
    }

    fn parse_value(&self, candidate: Option<&Token>) -> Option<Value> {
        candidate.and_then(|token| match token.code {
            TokenCode::Value1 => Some(Value::One),
            TokenCode::Value2 => Some(Value::Two),
            TokenCode::Value3 => Some(Value::Three),
            TokenCode::Value4 => Some(Value::Four),
            TokenCode::Value5 => Some(Value::Five),
            TokenCode::Value6 => Some(Value::Six),
            TokenCode::Value7 => Some(Value::Seven),
            TokenCode::Value8 => Some(Value::Eight),
            TokenCode::ValueInfinite => Some(Value::Infinity),
            _ => None,
        })
    }

    /// Finds the method parameter token for the given method type. The predicate is used to ensure
    /// the input type is as expected (condition vs value).
    fn find_method_parameter_token(
        &self,
        token: &Token,
        predicate: &impl Fn(&Token) -> bool,
    ) -> Option<&Token> {
        log::debug!("Trying to parse parameter from token: {:?}", token);
        let ratio = token.ratio(TOPCODE_DIAMETER);
        let distance = ratio * -TOKEN_SIZE;
        let x = token.x + distance * -token.orientation.sin();
        let y = token.y + distance * token.orientation.cos();
        for candidate in &self.tokens {
            if token == candidate || !predicate(candidate) {
                continue;
            }

            if Self::within_threshold(candidate, x, y, token.orientation) {
                return Some(candidate);
            }
        }
        None
    }

    fn find_method_body_token(&self, token: &Token) -> Option<&Token> {
        let ratio = token.ratio(TOPCODE_DIAMETER);
        let angle = token.orientation + PI / 2.0;
        let x_delta = -(TOPCODE_CENTER_X - TOPCODE_CENTER_Y);
        let y_delta = TOKEN_SIZE + x_delta;
        let cos_angle = token.orientation.cos();
        let sin_angle = token.orientation.sin();
        let x = token.x + (x_delta * cos_angle - y_delta * sin_angle) * ratio;
        let y = token.y + (x_delta * sin_angle + y_delta * cos_angle) * ratio;

        for candidate in &self.tokens {
            if token == candidate || !candidate.is_flow() {
                continue;
            }

            if Self::within_threshold(candidate, x, y, angle) {
                return Some(candidate);
            }
        }

        None
    }

    /// Given a flow token, find the token which is adjacent to it.
    fn find_adjacent_token(&self, token: &Token, parent: Option<&Token>) -> Option<&Token> {
        let ratio = token.ratio(TOPCODE_DIAMETER);
        let distance = ratio * TOKEN_SIZE;
        let x = token.x + (distance * token.orientation.cos());
        let y = token.y + (distance * token.orientation.sin());

        for candidate in &self.tokens {
            if candidate == token || !candidate.is_flow() {
                continue;
            }

            if let Some(parent) = parent {
                if parent == candidate {
                    continue;
                }
            }

            if Self::within_threshold(candidate, x, y, token.orientation) {
                return Some(candidate);
            }
        }

        None
    }

    fn within_threshold(candidate: &Token, x: f64, y: f64, angle: f64) -> bool {
        let delta_displacement_squared = (candidate.x - x).powi(2) + (candidate.y - y).powi(2);
        let mut delta_angle = (candidate.orientation - angle) % TWO_PI;
        if delta_angle < 0.0 {
            delta_angle += TWO_PI;
        }
        delta_angle = f64::min(delta_angle, TWO_PI - delta_angle);
        log::debug!(
            "within_threshold: {{\n  candidate: {:?}\n  expected_x: {}\n  expected_y: {}\n  expected_angle: {}\n  delta_displacement_squared: {}\n  delta_angle: {}\n  evaluation: {}\n}}",
            candidate,
            x,
            y,
            angle,
            delta_displacement_squared,
            delta_angle,
            delta_displacement_squared < DISPLACEMENT_SQUARED_TOLERANCE && delta_angle < ANGLE_TOLERANCE
        );
        delta_displacement_squared <= DISPLACEMENT_SQUARED_TOLERANCE
            && delta_angle <= ANGLE_TOLERANCE
    }

    fn find_true_token(&self, token: &Token) -> Option<&Token> {
        log::debug!("Trying to find true token...");
        let ratio = token.ratio(TOPCODE_DIAMETER);
        let angle = token.orientation + PI / 4.0;
        let cos_angle = token.orientation.cos();
        let sin_angle = token.orientation.sin();
        let x = token.x + (*TRUE_TOKEN_X * cos_angle - *TRUE_TOKEN_Y * sin_angle) * ratio;
        let y = token.y + (*TRUE_TOKEN_X * sin_angle + *TRUE_TOKEN_Y * cos_angle) * ratio;
        // Create an artificial to find the next flow token
        let pseudo_token = Token::new(TokenCode::Undefined, token.diameter, angle, x, y);
        log::debug!("Pseudo true token: {:?}", pseudo_token);
        self.find_adjacent_token(&pseudo_token, Some(token))
    }

    fn find_false_token(&self, token: &Token) -> Option<&Token> {
        log::debug!("Trying to find false token...");
        let ratio = token.ratio(TOPCODE_DIAMETER);
        let angle = token.orientation - PI / 4.0;
        let cos_angle = token.orientation.cos();
        let sin_angle = token.orientation.sin();
        let x = token.x + (*FALSE_TOKEN_X * cos_angle - *FALSE_TOKEN_Y * sin_angle) * ratio;
        let y = token.y + (*FALSE_TOKEN_X * sin_angle + *FALSE_TOKEN_Y * cos_angle) * ratio;
        // Create an artificial to find the next flow token
        let pseudo_token = Token::new(TokenCode::Undefined, token.diameter, angle, x, y);
        log::debug!("Pseudo false token: {:?}", pseudo_token);
        self.find_adjacent_token(&pseudo_token, Some(token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

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

    #[test]
    fn it_can_parse_a_complex_tree() {
        init();
        // This is essentially a fuzz test as there are a few missing unit tests above. This covers
        // quite a complex (although semantically nonsense) Tangibl AST.
        let topcodes = vec![
            TopCode::mock(61, 12.375, -5.244, 237.5, 165.166),
            TopCode::mock(79, 14.306, -5.244, 336.333, 363.5),
            TopCode::mock(31, 12.15, -5.244, 431.0, 562.0),
            TopCode::mock(47, 12.262, -2.827, 937.833, 377.5),
            TopCode::mock(91, 12.943, -1.232, 1140.5, 436.0),
            TopCode::mock(115, 14.175, -1.280, 1345.5, 519.5),
            TopCode::mock(59, 12.706, -1.280, 1052.5, 641.666),
            TopCode::mock(55, 13.912, -5.920, 801.5, 754.833),
            TopCode::mock(91, 13.912, -5.969, 999.0, 838.0),
            TopCode::mock(167, 12.468, -4.422, 134.5, 912.833),
            TopCode::mock(87, 12.15, -5.969, 1201.666, 916.0),
            TopCode::mock(155, 13.125, -4.422, 336.833, 979.5),
            TopCode::mock(47, 12.35, -5.969, 538.0, 1031.5),
            TopCode::mock(117, 13.25, -5.969, 917.666, 1035.5),
            TopCode::mock(87, 11.812, -4.47, 275.33, 1177.33),
        ];

        let parser = Parser::new(&topcodes);

        assert_eq!(
            Some(Start {
                next: Some(Flow {
                    kind: FlowKind::Command(Command::TurnLeft),
                    next: Some(Box::new(Flow {
                        kind: FlowKind::Conditional(Conditional {
                            kind: ConditionalKind::Blocked,
                            alternate: Some(Box::new(Flow {
                                kind: FlowKind::BooleanMethod(BooleanMethod {
                                    kind: BooleanMethodKind::While,
                                    condition: Some(Condition::IsPathClear),
                                    body: Some(Box::new(Flow {
                                        kind: FlowKind::Command(Command::MoveBackwards),
                                        next: None
                                    }))
                                }),
                                next: Some(Box::new(Flow {
                                    kind: FlowKind::Command(Command::TurnRight),
                                    next: None,
                                }))
                            }))
                        }),
                        next: Some(Box::new(Flow {
                            kind: FlowKind::Command(Command::MoveForwards),
                            next: Some(Box::new(Flow {
                                kind: FlowKind::IntegerMethod(IntegerMethod {
                                    kind: IntegerMethodKind::Repeat,
                                    body: Some(Box::new(Flow {
                                        kind: FlowKind::Command(Command::Shoot),
                                        next: Some(Box::new(Flow {
                                            kind: FlowKind::IntegerMethod(IntegerMethod {
                                                kind: IntegerMethodKind::Repeat,
                                                body: Some(Box::new(Flow {
                                                    kind: FlowKind::Command(Command::MoveBackwards),
                                                    next: None,
                                                })),
                                                value: Some(Value::Five)
                                            }),
                                            next: None
                                        })),
                                    })),
                                    value: Some(Value::Six)
                                }),
                                next: Some(Box::new(Flow {
                                    kind: FlowKind::Command(Command::TurnRight),
                                    next: None
                                }))
                            }))
                        }))
                    }))
                })
            }),
            parser.parse()
        );
    }
}
