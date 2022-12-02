use std::collections::HashSet;
use std::fs;
use std::thread::current;

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

struct Tokenizer {
    state: State,
    tokens:Vec<Token>,
    current_char: usize,
    start_char: usize,
    contents: String
}

impl Tokenizer {
    fn create_tokenizer(contents: String) -> Tokenizer {
        let mut state = State::Start;
        let mut tokens:Vec<Token> = Vec::new();

        Tokenizer { 
            state: state,
            tokens: tokens,
            current_char: 0,
            start_char: 0,
            contents: contents
        }
    }

    fn tokenize(&mut self) {
        while self.tokens_exist() {
            match self.state {
                State::Start => {
                    self.next_character();
                    if is_operator(self.get_current_char()) {
                        self.transition(State::Operator);
                    }
                    if is_seperator(self.get_current_char()) {
                        self.transition(State::Seperator);
                    }
                    if is_alpha(self.get_current_char()) {
                        self.transition(State::Identifier);
                    }
                    if is_number(self.get_current_char()) {
                        self.transition(State::Number)
                    }
                },
                State::Final => {
                    self.start_char = self.current_char;
                    self.state.transition(State::Start);
                },
                State::Operator => {
                    if is_operator(self.look_ahead()) {
                        self.next_character();
                    } else {
                        self.tokens.push(self.create_token(TokenType::Operator, self.get_window()));
                        self.transition(State::Final);
                    }
                },
                State::Seperator => {
                    self.tokens.push(self.create_token(TokenType::Seperator, self.get_window()));
                    self.transition(State::Final);
                },
                State::Identifier => {
                    if is_alphanumeric(self.look_ahead()) {
                        self.next_character();
                    }
                    else if is_keyword(self.get_window()) {
                        self.transition(State::Keyword);
                    }
                    else {
                        let x = self.get_window();
                        self.tokens.push(self.create_token(TokenType::Identifier, self.get_window()));
                        self.transition(State::Final);
                    }
                },
                State::Keyword => {
                    self.tokens.push(self.create_token(TokenType::Keyword, self.get_window()));
                    self.transition(State::Final);
                },
                State::Number => {
                    if is_number(self.look_ahead())  {
                        self.next_character();
                    }
                    else if self.look_ahead() == "." {
                        self.transition(State::Decimal);
                        self.next_character();
                    }
                    else {
                        self.tokens.push(self.create_token(TokenType::Number, self.get_window()));
                        self.transition(State::Final);
                    }
                },
                State::Decimal => {
                    if is_number(self.look_ahead())  {
                        self.next_character();
                    }
                    else {
                        self.tokens.push(self.create_token(TokenType::Decimal, self.get_window()));
                        self.transition(State::Final);
                    }
                }
            }
        }
    }

    //TODO Handle unwrap
    fn tokens_exist(&self) -> bool {
        self.current_char < self.contents.len().try_into().unwrap() || !matches!(self.state, State::Final)
    }

    fn next_character(&mut self) {
        self.current_char += 1;
    }

    //TODO Catch overflows
    fn look_ahead(&self) -> &str {
        self.get_char_at(self.current_char, self.current_char + 1)
    }

    fn get_tokens(self) -> Vec<Token> {
        self.tokens
    }

    fn transition(&mut self, state: State) {
        self.state.transition(state);
    }

    fn get_current_char(&self) -> &str {
        self.get_char_at(self.current_char - 1, self.current_char)
    }

    fn get_window(&self) -> &str {
        self.get_char_at(self.start_char, self.current_char)
    }

    fn get_char_at(&self, start:usize, end:usize) -> &str {
        match self.contents.get(start..end) {
            Some(slice) => {
                slice
            },
            None => {
                ""
            }
        }
    }

    fn create_token(&self, token_type: TokenType, value: &str) -> Token {
        let token_value =  String::from(str::replace(value,"\n",""));
        Token {
            token_type: token_type,
            value: token_value
        }
    }

}

#[derive(Debug)]
enum State {
    Start,
    Final,
    Operator,
    Seperator,
    Identifier,
    Keyword,
    Number,
    Decimal
}

impl State {
    fn transition(&mut self, transition_state: State) {
        *self = transition_state;
    }
}

#[derive(Debug)]
enum TokenType {
    Keyword,
    Identifier,
    Operator,
    Seperator,
    Number,
    Decimal
}

#[derive(Debug)]
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

fn is_alphanumeric(current_char: &str) -> bool {
    for character in current_char.chars() {
        return character.is_alphanumeric();
    }
    return false;
}

//TODO Put these in a constructor so they are only initialized once
fn is_operator(current_char: &str) -> bool {
    let keyword = HashSet::from(["=","==","<",">","<=",">=","!=","+","-","*","=>"]);
    return keyword.contains(current_char);
}

fn is_seperator(current_char: &str) -> bool {
    let keyword = HashSet::from(["(",")","{","}"," ",";"]);
    return keyword.contains(current_char);
}

fn clean() {

}