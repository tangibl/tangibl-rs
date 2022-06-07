use serde_json::{Map, Value};

use crate::{nodes::Start, Visitor};

pub struct JsonPrinter {}

impl JsonPrinter {
    fn print(&mut self, start: &Start) -> Value {
        let root = Value::Object(Map::new());
        self.visit_start(start, root)
    }
}

impl Visitor for JsonPrinter {
    type Args = Value;

    fn visit_start(&mut self, start: &Start, jso: Self::Args) -> Self::Args {
        jso
    }
}
