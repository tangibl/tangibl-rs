mod start;

use crate::Visitor;
pub use start::Start;
use std::{borrow::Cow, fmt::Debug};

pub trait Node {
    fn name(&self) -> Cow<'_, str>;
}

pub(crate) trait AstNode: Node {
    fn apply<V: Visitor + ?Sized>(&self, visitor: &mut V, args: V::Args) -> V::Args;
}

pub trait FlowNode: Node {
    fn prev(&self) -> Option<Box<&dyn FlowNode>>;

    fn next(&self) -> Option<Box<&dyn FlowNode>>;
}

impl Debug for dyn FlowNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
