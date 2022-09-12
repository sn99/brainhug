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

fn generate(tokens: &[Token]) -> String {
    let mut has_stdin_interraction = false;

    let mut output = String::default();
    let mut indent = 1;

    for &token in tokens {
        match token {
            Add => {
                for _ in 0..indent {
                    output.push('\t');
                }
                // Increment the value at the selected cell
                output.push_str("ptr[ndx] += 1\n")
            }
            Sub => {
                for _ in 0..indent {
                    output.push('\t');
                }
                // Decrement the value at the selected cell
                output.push_str("ptr[ndx] -= 1\n")
            }
            Right => {
                for _ in 0..indent {
                    output.push('\t');
                }
                // Change our selected cell to the next to the right
                output.push_str("ndx += 1\n");
            }
            Left => {
                for _ in 0..indent {
                    output.push('\t');
                }
                // Change our selected cell to the next to the left
                output.push_str("ndx -= 1\n");
            }
            Read => {
                for _ in 0..indent {
                    output.push('\t');
                }
                // Read a single character into the selected cell
                output.push_str("ptr[ndx] = getchar(reader)\n");
                has_stdin_interraction = true;
            }
            Write => {
                for _ in 0..indent {
                    output.push('\t');
                }
                // Print the character at the selected cell
                output.push_str("fmt.Printf(\"%c\", ptr[ndx])\n")
            }
            BeginLoop => {
                indent += 1;

                for _ in 0..(indent - 1) {
                    output.push('\t');
                }
                // Begin a loop at the current cell
                output.push_str("for (ptr[ndx] != 0) {\n");
            }
            EndLoop => {
                indent -= 1;

                for _ in 0..indent {
                    output.push('\t');
                }
                // Close a loop
                output.push_str("}\n");
            }
        }
    }
    output.push_str("}\n");

    let mut header = if has_stdin_interraction {
        String::from(include_str!("preface_stdin.go"))
    } else {
        String::from(include_str!("preface.go"))
    };
    header.push_str(&output);

    header
}

/// generate string of C++ code from a Brainf*ck string
pub fn brains(input: &str) -> String {
    let tokens = tokenize(input);

    generate(&tokens)
}
