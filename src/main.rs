mod tokenizer;
mod ast;

fn main() {
    let input = std::fs::read_to_string("file.ll").unwrap();
    let tokenizer = tokenizer::Tokenizer::new(input);
    let tokens = tokenizer.tokenize();

    let ast = ast::Parser::new(tokens).parse_program();

    println!("{:#?}", ast);
}
