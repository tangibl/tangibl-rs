use tangibl::{
    ast::{BooleanMethodKind, Command, Condition, ConditionalKind, IntegerMethodKind, Value},
    visitors::JsonPrinter,
    Tangibl,
};

fn main() {
    let ast = Tangibl::start()
        .with_command(Command::Shoot)
        .with_conditional(
            ConditionalKind::Blocked,
            Tangibl::flow().with_command(Command::TurnLeft).build(),
        )
        .with_command(Command::MoveBackwards)
        .with_boolean_method(
            BooleanMethodKind::While,
            Some(Condition::IsBlocked),
            Tangibl::flow().with_command(Command::TurnRight).build(),
        )
        .with_command(Command::MoveForwards)
        .with_integer_method(
            IntegerMethodKind::Repeat,
            Some(Value::Three),
            Tangibl::flow().with_command(Command::TurnLeft).build(),
        )
        .with_command(Command::MoveForwards)
        .build();
    let mut json_printer = JsonPrinter::new();

    println!("{}", json_printer.print(&ast));
}
