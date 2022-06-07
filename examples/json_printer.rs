use tangibl::Tangibl;

fn main() {
    let ast = Tangibl::builder().build();
    println!("{:?}", ast);
}
