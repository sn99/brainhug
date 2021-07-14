
// Interpreted code is intended to run in your web browser,
// support for NodeJS is not implemented and probably won't
// be implemented

// Returns HTML, so that you can instantly run this program
// in your browser. If you don't need to do it, you can extract
// compiled JavaScript code from HTML in a text editor

// Tested with Firefox 89

use super::tokenize;
use super::Token;
use super::Token::*;

fn generate(tokens: &[Token]) -> String {
    let mut output = String::from(include_str!("preface.html"));
    let mut indent = 1;

    for &token in tokens {
        match token {
            Add => {
                output += &("  ".repeat(indent) + "tape[index] += 1;\n")
            }
            Sub => {
                output += &("  ".repeat(indent) + "tape[index] -= 1;\n")
            }
            Right => {
                output += &("  ".repeat(indent) + "index += 1;\n")
            }
            Left => {
                output += &("  ".repeat(indent) + "index -= 1;\n")
            }
            Read => {
                output += &("  ".repeat(indent) + "tape[index] = read();\n")
            }
            Write => {
                output += &("  ".repeat(indent) + "write(tape[index]);\n")
            }
            BeginLoop => {
                output += &("  ".repeat(indent) + "while (tape[index]) {\n");
                indent += 1;
            }
            EndLoop => {
                indent -= 1;
                output += &("  ".repeat(indent) + "}\n");
            }
        }
    }

    output.push_str(include_str!("postface.html"));

    output
}

/// Generate string of JavaScript, embedded in HTML, from a Brainf*ck string
pub fn brains(input: &str) -> String {
    let tokens = tokenize(input);

    generate(&tokens)
}
