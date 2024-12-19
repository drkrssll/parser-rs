mod parser;
mod token;

use parser::Parser;

fn main() {
    let mut parser = Parser::new();

    let result = parser.parse("print 123");

    println!("{:?}", result);
}
