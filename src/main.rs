use std::collections::HashSet;
use std::fs;
use std::thread::current;

fn main() {
    let mut state = State::Scanning;

    let file_path = "hello.jnk";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut start = 0;
    let mut end = 1;

    let mut tokens:Vec<Token> = Vec::new();

    while end <= contents.len() {
        let slice = String::from(&contents[start..end]);
        let current = &contents[end-1..end];

        match state {
            State::Scanning => {
                if is_alpha(&slice) {
                    state = State::Identifier;
                } else if is_number(current) {
                    state = State::Int;
                } else if is_operator(current) {
                    state = State::Operator;
                }

                if is_end_of_token(current) {
                    state.is_end_of_token();
                    continue;
                }


                if current == "\"" {
                    state = State::Str;
                }

                if current == "'" {
                    state = State::Char;
                }

            },
            State::Int => {
                if current == "." {
                    state = State::Double;
                }
                if is_end_of_token(current) {
                    state.is_end_of_token();
                    tokens.push(
                        Token {
                            token_type: TokenType::Int,
                            value: clean_string(&slice)
                        }
                    );
                    continue;                
                }
            },
            State::Double => {
                if is_end_of_token(current) {
                    state.is_end_of_token();
                    tokens.push(
                        Token {
                            token_type: TokenType::Double,
                            value: clean_string(&slice)
                        }
                    );
                    continue;                
                }
            }
            State::Identifier => {
                if is_keyword(&slice) {
                    state = State::Keyword;
                    end += 1;
                    continue;
                }
                if is_end_of_token(current) {
                    state.is_end_of_token();
                    tokens.push(
                        Token {
                            token_type: TokenType::Identifier,
                            value: clean_string(&slice)
                        }
                    );
                    continue;                
                }
            }
            State::Str => {
                if current == "\"" {
                    state = State::Scanning;
                    state.is_end_of_token();
                    tokens.push(
                        Token {
                            token_type: TokenType::Str,
                            //Ignore opening and closing quotes
                            value: String::from(&contents[start+1..end-1])
                        }
                    );
                    continue;
                }
            },
            State::Char => {
                if current == "'" {
                    state = State::Scanning;
                    state.is_end_of_token();
                    tokens.push(
                        Token {
                            token_type: TokenType::Char,
                            //Ignore opening and closing quotes
                            value: String::from(&contents[start+1..end-1])
                        }
                    );
                    continue;
                }
            }
            State::Operator => {
                if is_end_of_token(current) || is_alpha(current) || is_number(current) {
                    state.is_end_of_token();
                    tokens.push(
                        Token {
                            token_type: TokenType::Operator,
                            value: clean_string(&slice)
                        }
                    );
                    continue;             
                } else if is_operator(current) {
                    
                }
                else {
                    state = State::Identifier;
                }
            },
            State::Seperator => {

            },
            State::Keyword => {
                if is_end_of_token(current) {
                    state.is_end_of_token();
                    tokens.push(
                        Token {
                            token_type: TokenType::Keyword,
                            value: clean_string(&slice)
                        }
                    );
                    continue;                
                } else {
                    state = State::Identifier;
                }
            },
            State::EndOfToken => {
                start = end;
                state.reset();
            }
        }

        println!("{:?}", state);
        println!("{:?}", slice);

        end += 1;
    }

    let x = 1;
}

#[derive(Debug)]
enum State {
    Scanning,
    Str,
    Keyword,
    Char,
    Int,
    Double,
    Identifier,
    Operator,
    Seperator,
    EndOfToken
}

impl State {
    fn reset(&mut self) {
        *self = State::Scanning;
    }

    fn is_end_of_token(&mut self) {
        *self = State::EndOfToken;
    }
}

enum TokenType {
    Keyword,
    Str,
    Identifier,
    Int,
    Double,
    Char,
    Operator
}

struct Token {
    token_type: TokenType,
    value: String
}

fn is_keyword(current_char:&str) -> bool {
    let keyword = HashSet::from(["int","double", "char", "str"]);
    return keyword.contains(current_char);
}

fn is_end_of_token(current_char: &str) -> bool {
    if current_char == " " || current_char == "\n" || current_char == ";" {
        return true;
    }
    false
}

fn is_number(current_char: &str) -> bool {
    for character in current_char.chars() {
        return character.is_numeric();
    }
    return false;
}

fn is_alpha(current_char: &str) -> bool {
    for character in current_char.chars() {
        return character.is_alphabetic();
    }
    return false;
}

fn is_operator(current_char: &str) -> bool {
    let keyword = HashSet::from(["=","==","<",">","<=",">=","!="]);
    return keyword.contains(current_char);
}

fn clean_string(current_slice:&str) -> String{
    current_slice.replace(" ", "").replace(";", "")
}
