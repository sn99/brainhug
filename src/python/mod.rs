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

use super::tokenize;
use super::Token;
use super::Token::*;

const BASE_INDENT: &'static str = "    ";

enum State {
    Alter(i64),
    Move(i64),
}

fn get_indent(indent: u32) -> String {
    let mut indent_str = String::from(BASE_INDENT);
    for _ in 1..indent {
        indent_str.push_str(BASE_INDENT)
    }
    indent_str
}

fn flush(state: &State, output: &mut String, indent: &String) {
    output.push_str(&indent[..]);
    match state {
        State::Alter(x) if x > &0 => output.push_str(&format!("tape[index] += {}\n", x)[..]),
        State::Alter(x) if x < &0 => output.push_str(&format!("tape[index] -= {}\n", -x)[..]),
        State::Move(x) if x > &0 => output.push_str(&format!("index += {}\n", x)[..]),
        State::Move(x) if x < &0 => output.push_str(&format!("index -= {}\n", -x)[..]),
        _ => (), // x == 0, which is a no-op
    }
}

fn generate(tokens: &[Token]) -> String {
    let mut output = String::from(include_str!("preface.py"));
    let mut indent_index = 1;
    let mut indent = get_indent(indent_index);
    let mut state: Option<State> = None;
    // tokens that cannot accumulate state
    let non_state_tokens = [Read, Write, BeginLoop, EndLoop];

    for &token in tokens {
        if non_state_tokens.contains(&token) {
            if let Some(s) = state {
                flush(&s, &mut output, &indent);
                state = None;
            }
        }
        match token {
            Add => {
                state = Some(match state {
                    Some(State::Alter(i)) => State::Alter(i + 1),
                    None => State::Alter(1),
                    Some(s) => {
                        flush(&s, &mut output, &indent);
                        State::Alter(1)
                    }
                })
            }
            Sub => {
                state = Some(match state {
                    Some(State::Alter(i)) => State::Alter(i - 1),
                    None => State::Alter(-1),
                    Some(s) => {
                        flush(&s, &mut output, &indent);
                        State::Alter(-1)
                    }
                })
            }
            Right => {
                state = Some(match state {
                    Some(State::Move(i)) => State::Move(i + 1),
                    None => State::Move(1),
                    Some(s) => {
                        flush(&s, &mut output, &indent);
                        State::Move(1)
                    }
                })
            }
            Left => {
                state = Some(match state {
                    Some(State::Move(i)) => State::Move(i - 1),
                    None => State::Move(-1),
                    Some(s) => {
                        flush(&s, &mut output, &indent);
                        State::Move(-1)
                    }
                })
            }
            Read => {
                output.push_str(&indent[..]);
                // Read a single character into the selected cell
                output.push_str("tape[index] = read_char()\n");
            }
            Write => {
                output.push_str(&indent[..]);
                // Print the character at the selected cell
                output.push_str("write_char(tape[index])\n");
            }
            BeginLoop => {
                output.push_str(&indent[..]);
                // Begin a loop at the current cell
                output.push_str("while tape[index]:\n");
                indent_index += 1;
                indent = get_indent(indent_index);
            }
            EndLoop => {
                indent_index -= 1;
                indent = get_indent(indent_index);
            }
        }
    }

    if let Some(s) = state {
        flush(&s, &mut output, &indent);
    }

    output.push_str(include_str!("postface.py"));

    output
}

/// generate string of C++ code from a Brainf*ck string
pub fn brains(input: &str) -> String {
    let tokens = tokenize(input);

    generate(&tokens)
}
