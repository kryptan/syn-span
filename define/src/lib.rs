use proc_macro2::LineColumn;
use syn::{
    parse::{ParseStream, Parser, Result},
    Ident,
};

#[proc_macro]
pub fn m(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let from_input = Parser::parse(parse, input).unwrap();
    let from_str = Parser::parse_str(parse, "a b").unwrap();

    dbg!(&from_input);
    assert!(from_input[0] != from_input[1]);

    dbg!(&from_str);
    assert!(from_str[0] != from_str[1]);

    proc_macro::TokenStream::new()
}

fn parse(tokens: ParseStream) -> Result<[LineColumn; 2]> {
    let a = tokens.parse::<Ident>()?.span().start();
    let b = tokens.parse::<Ident>()?.span().start();

    Ok([a, b])
}
