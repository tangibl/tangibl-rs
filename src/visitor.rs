use crate::ast::{BooleanMethod, Command, Conditional, Flow, IntegerMethod, Start};

pub(crate) trait Visitor {
    type Result;

    fn visit_start(&mut self, start: &Start) -> Self::Result;
    fn visit_flow(&mut self, flow: &Flow) -> Self::Result;
    fn visit_command(&mut self, command: &Command) -> Self::Result;
    fn visit_boolean_method(&mut self, boolean_method: &BooleanMethod) -> Self::Result;
    fn visit_integer_method(&mut self, integer_method: &IntegerMethod) -> Self::Result;
    fn visit_conditional(&mut self, conditional: &Conditional) -> Self::Result;
}
