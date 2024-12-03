use std::{borrow::Borrow, ops::Index};

use crate::Application;

#[derive(Debug, PartialEq)]
enum TokenType {
    Keyword,
    OpenParen,
    Number,
    Comma,
    CloseParen,
    Other
}

#[derive(Debug, PartialEq)]
struct Token {
    token_type: TokenType,
    value: String,
}

impl Token {
    fn new(token_type: TokenType, value: String) -> Self {
        Token { token_type, value }
    }
}

struct Tokenizer {
    input: String,
    position: usize,
}

impl Tokenizer {
    fn new(input: String) -> Self {
        Tokenizer { input, position: 0 }
    }

    fn get_next_token(&mut self) -> Option<Token> {
        if self.position >= self.input.len() {
            return None;
        }
        let current_char = self.input.chars().nth(self.position).unwrap();
        if current_char == 'm' {
            return self.collect_keyword();
        } else if current_char.is_numeric() {
            return self.collect_number();
        } else if current_char == ',' {
            self.position += 1;
            return Some(Token::new(TokenType::Comma, ','.to_string()))
        } else if current_char == '(' {
            self.position += 1;
            return Some(Token::new(TokenType::OpenParen, '('.to_string()))
        } else if current_char == ')' {
            self.position += 1;
            return Some(Token::new(TokenType::CloseParen, ')'.to_string()))
        } else {
            self.position += 1;
            return Some(Token::new(TokenType::Other, current_char.to_string()));
        }
    }

    fn collect_keyword(&mut self) -> Option<Token> {
        let start_pos = self.position;
        while self.position < self.input.len() && self.input.chars().nth(self.position).unwrap().is_alphabetic() {
            self.position += 1;
        }
        let word = self.input[start_pos..self.position].to_string();
        if word == "mul".to_string() {
            return Some(Token::new(TokenType::Keyword, word));
        }
        return Some(Token::new(TokenType::Other, word));
    }

    fn collect_number(&mut self) -> Option<Token> {
        let start_pos = self.position;
        while self.position < self.input.len() && self.input.chars().nth(self.position).unwrap().is_numeric() {
            self.position += 1;
        }
        Some(Token::new(TokenType::Number, self.input[start_pos..self.position].to_string()))
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.get_next_token() {
            tokens.push(token);
        }
        tokens
    }

}

impl Application {
    pub fn day3(self) {
        if self.args.part == 1 {
            self.d3p1();
        } else {
            self.d3p2();
        }
    }

    fn d3p1(self) {
        let giant_string = self.input.into_iter().collect::<String>();
        let mut answer = 0;
        let mut tokenizer = Tokenizer::new(giant_string);
        let tokens = tokenizer.tokenize();
        for (idx, token) in tokens.iter().enumerate() {
            if idx > (tokens.len() - 6) {
                break;
            }
            if token.token_type == TokenType::Keyword {
                if check_sequence(tokens[idx..=(idx + 5)].try_into().unwrap()) {
                    answer += tokens[idx + 2].value.parse::<i32>().unwrap() * tokens[idx + 4].value.parse::<i32>().unwrap();
                }
            }
        }
        println!("{answer}");
    }

    fn d3p2(self) {

    }
}

fn check_sequence(next_six: &[Token;6]) -> bool {
    if next_six[0].token_type != TokenType::Keyword {
        return false;
    }
    if next_six[1].token_type != TokenType::OpenParen {
        return false;
    }
    if next_six[2].token_type != TokenType::Number {
        return false;
    }
    if next_six[3].token_type != TokenType::Comma {
        return false;
    }
    if next_six[4].token_type != TokenType::Number {
        return false;
    }
    if next_six[5].token_type != TokenType::CloseParen {
        return false;
    }
    return true;
}
