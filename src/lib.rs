/*
MIT License

Copyright (c) 2019 Siddharth Naithani

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

//! # brainhug
//!
//! `brainhug` is a crate that is used to interpret brainf*ck code to any
//! other language
//!
//! #### Current languages covered are :
//!- [x] C
//!- [x] C++

pub mod c;
pub mod cpp;

/// tokens for brainf*ck
#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    Add,        // +
    Sub,        // -
    Right,      // >
    Left,       // <
    Read,       // ,
    Write,      // .
    BeginLoop,  // [
    EndLoop,    // ]
}

use self::Token::*;

/// Assign enum values to string chars of brainf*ck
fn tokenize(input: &str) -> Vec<Token> {
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

/// languages covered by the crate
/// Use `Lang::` to  specify a language to be used
pub enum Lang {
    C,    // C language
    Cpp   // C++ language

}

use self::Lang::*;

/// main function to be called in `main.rs`
pub fn generate(lang: Lang, input: &str) -> String {
    match lang {
        C => {
            use crate::c::brains;
            brains(input).to_string()
        },
        Cpp => {
            use crate::cpp::brains;
            brains(input).to_string()
        },
        _ => {
            input.to_string()
        }
    }
}