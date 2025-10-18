use chumsky::prelude::*;

fn simple_parser() -> impl Parser<char, String, extra::Err<Rich<char>>> {
    text::ident()
}

fn main() {
    let result = simple_parser().parse("hello");
    println!("{:?}", result);
}
