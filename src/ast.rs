#[derive(Clone, Default, Debug)]
pub struct Start {
    pub next: Option<Flow>,
}

#[derive(Clone, Copy, Debug)]
pub enum Condition {
    IsBlocked,
    IsPathClear,
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
pub enum Command {
    MoveBackwards,
    MoveForwards,
    Shoot,
    TurnLeft,
    TurnRight,
}

#[derive(Clone, Copy, Debug)]
pub enum BooleanMethodKind {
    While,
}

#[derive(Clone, Debug)]
pub struct BooleanMethod {
    pub kind: BooleanMethodKind,
    pub body: Option<Box<Flow>>,
    pub condition: Option<Condition>,
}

#[derive(Clone, Copy, Debug)]
pub enum IntegerMethodKind {
    Repeat,
}

#[derive(Clone, Debug)]
pub struct IntegerMethod {
    pub kind: IntegerMethodKind,
    pub body: Option<Box<Flow>>,
    pub value: Option<Value>,
}

#[derive(Clone, Copy, Debug)]
pub enum ConditionalKind {
    Blocked,
}

#[derive(Clone, Debug)]
pub struct Conditional {
    pub kind: ConditionalKind,
    /// The false path
    pub alternate: Option<Box<Flow>>,
}

#[derive(Clone, Debug)]
pub enum FlowKind {
    Command(Command),
    BooleanMethod(BooleanMethod),
    IntegerMethod(IntegerMethod),
    Conditional(Conditional),
}

#[derive(Clone, Debug)]
pub struct Flow {
    pub kind: FlowKind,
    pub next: Option<Box<Flow>>,
}

impl Flow {
    pub fn new(kind: FlowKind) -> Self {
        Self { kind, next: None }
    }
}
