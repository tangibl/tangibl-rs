#[cfg(test)]
use enum_iterator::Sequence;
use num_enum::TryFromPrimitive;

#[cfg_attr(test, derive(Sequence))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u32)]
pub enum TokenCode {
    Start = 61,

    // Flow
    // Conditional
    Blocked = 31,
    // Commands
    MoveBackwards = 47,
    MoveForwards = 55,
    Shoot = 59,
    TurnLeft = 79,
    TurnRight = 87,
    // Integer Methods
    Repeat = 91,
    // Boolean Methods
    While = 155,

    // Integer values
    Value1 = 93,
    Value2 = 103,
    Value3 = 107,
    Value4 = 109,
    Value5 = 115,
    Value6 = 117,
    Value7 = 121,
    Value8 = 143,
    ValueInfinite = 151,

    // Conditions
    IsBlocked = 157,
    IsPathClear = 167,

    // An undefined token value
    Undefined = 0,
}

impl TokenCode {
    pub fn value(&self) -> u32 {
        *self as u32
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Token {
    pub code: TokenCode,
    pub diameter: f64,
    pub orientation: f64,
    pub x: f64,
    pub y: f64,
}

impl Token {
    pub fn new(code: TokenCode, diameter: f64, orientation: f64, x: f64, y: f64) -> Self {
        Self {
            code,
            diameter,
            orientation,
            x,
            y,
        }
    }

    /// Is part of the regular flow of the program. In other words, has a previous and next token.
    pub fn is_flow(&self) -> bool {
        match self.code {
            TokenCode::Blocked
            | TokenCode::MoveBackwards
            | TokenCode::MoveForwards
            | TokenCode::Shoot
            | TokenCode::TurnLeft
            | TokenCode::TurnRight
            | TokenCode::Repeat
            | TokenCode::While => true,
            _ => false,
        }
    }

    pub fn is_command(&self) -> bool {
        match self.code {
            TokenCode::Shoot
            | TokenCode::TurnLeft
            | TokenCode::TurnRight
            | TokenCode::MoveForwards
            | TokenCode::MoveBackwards => true,
            _ => false,
        }
    }

    /// Represents a positive integer value.
    pub fn is_value(&self) -> bool {
        match self.code {
            TokenCode::Value1
            | TokenCode::Value2
            | TokenCode::Value3
            | TokenCode::Value4
            | TokenCode::Value5
            | TokenCode::Value6
            | TokenCode::Value7
            | TokenCode::Value8
            | TokenCode::ValueInfinite => true,
            _ => false,
        }
    }

    /// A method with a condition.
    pub fn is_condition(&self) -> bool {
        match self.code {
            TokenCode::IsBlocked | TokenCode::IsPathClear => true,
            _ => false,
        }
    }

    /// The ratio of the current token to the expected diameter.
    pub fn ratio(&self, diameter: f64) -> f64 {
        self.diameter / diameter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use enum_iterator::all;
    use topcodes::TopCode;

    #[test]
    fn tokencode_enum_values_are_valid_topcodes() {
        let tokens = all::<TokenCode>().collect::<Vec<_>>();
        for token in tokens {
            assert!(TopCode::checksum(token.value()) || token.value() == 0);
        }
    }
}
