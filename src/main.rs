mod tokenizer;

fn main() {
    // read input from file.ll
    let input = std::fs::read_to_string("file.ll").unwrap();
    // create a tokenizer
    let tokenizer = tokenizer::Tokenizer::new(input);
    // tokenize the input
    let tokens = tokenizer.tokenize();
    // print the tokens
    println!("{:?}", tokens);
}
