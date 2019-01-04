pub mod c;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    Add,
    // +
    Sub,
    // -
    Right,
    // >
    Left,
    // <
    Read,
    // ,
    Write,
    // .
    BeginLoop,
    // [
    EndLoop,    // ]
}

use self::Token::*;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();

    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        match c {
            '+' => tokens.push(Add),
            '-' => tokens.push(Sub),
            '>' => tokens.push(Right),
            '<' => tokens.push(Left),
            ',' => tokens.push(Read),
            '.' => tokens.push(Write),
            '[' => tokens.push(BeginLoop),
            ']' => tokens.push(EndLoop),
            _ => {}
        }
    }

    tokens
}

pub enum Lang {
    C // C language
}

use self::Lang::*;

pub fn generate(lang: Lang, input: &str) -> String {
    match lang {
        C => {
            use crate::c::brains;
            brains(input).to_string()
        }
        _ => {
            input.to_string()
        }
    }
}