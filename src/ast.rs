#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Start {
    pub next: Option<Flow>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Condition {
    IsBlocked,
    IsPathClear,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Value {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Infinity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    MoveBackwards,
    MoveForwards,
    Shoot,
    TurnLeft,
    TurnRight,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BooleanMethodKind {
    While,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BooleanMethod {
    pub kind: BooleanMethodKind,
    pub body: Option<Box<Flow>>,
    pub condition: Option<Condition>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntegerMethodKind {
    Repeat,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntegerMethod {
    pub kind: IntegerMethodKind,
    pub body: Option<Box<Flow>>,
    pub value: Option<Value>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConditionalKind {
    Blocked,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Conditional {
    pub kind: ConditionalKind,
    /// The false path
    pub alternate: Option<Box<Flow>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FlowKind {
    Command(Command),
    BooleanMethod(BooleanMethod),
    IntegerMethod(IntegerMethod),
    Conditional(Conditional),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Flow {
    pub kind: FlowKind,
    pub next: Option<Box<Flow>>,
}

impl Flow {
    pub fn new(kind: FlowKind) -> Self {
        Self { kind, next: None }
    }
}
