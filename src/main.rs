use std::fs;
pub use junk_lang::tokenizer::Tokenizer;
pub use junk_lang::tokenizer::Token;

fn main() {
    let file_path = "hello.jnk";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut tokenizer = Tokenizer::create_tokenizer(contents);
    tokenizer.tokenize();
    let tokens: Vec<Token> = tokenizer.get_tokens();

    for token in tokens {
        println!("{:?}",token);
    }
}
