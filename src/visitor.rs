use crate::nodes::Start;

pub(crate) trait Visitor {
    type Args;

    fn visit_start(&mut self, start: &Start, args: Self::Args) -> Self::Args;
}
