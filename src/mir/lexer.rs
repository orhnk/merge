// Licensed under MIT Open Source License

#[derive(Debug)]
struct Lexer<'a> {
    content: &'a str,
    cursor: &'a char,
}
