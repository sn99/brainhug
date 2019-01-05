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
use super::Token::*;
use super::Token;

fn generate(tokens: &[Token]) -> String {
    let mut output = String::from(include_str!("preface.cpp"));
    let mut indent = 1;

    for &token in tokens {
        match token {
            Add => {
                for _ in 0..indent {
                    output.push_str("\t");
                }
                // Increment the value at the selected cell
                output.push_str("++*ptr;\n");
            }
            Sub => {
                for _ in 0..indent {
                    output.push_str("\t");
                }
                // Decrement the value at the selected cell
                output.push_str("--*ptr;\n");
            }
            Right => {
                for _ in 0..indent {
                    output.push_str("\t");
                }
                // Change our selected cell to the next to the right
                output.push_str("++ptr;\n");
            }
            Left => {
                for _ in 0..indent {
                    output.push_str("\t");
                }
                // Change our selected cell to the next to the left
                output.push_str("--ptr;\n");
            }
            Read => {
                for _ in 0..indent {
                    output.push_str("\t");
                }
                // Read a single character into the selected cell
                output.push_str("*ptr = getchar();\n");
            }
            Write => {
                for _ in 0..indent {
                    output.push_str("\t");
                }
                // Print the character at the selected cell
                output.push_str("putchar(*ptr);\n");
            }
            BeginLoop => {
                indent = indent + 1;

                for _ in 0..(indent - 1) {
                    output.push_str("\t");
                }
                // Begin a loop at the current cell
                output.push_str("while (*ptr) {\n");
            }
            EndLoop => {
                indent = indent - 1;

                for _ in 0..indent {
                    output.push_str("\t");
                }
                // Close a loop
                output.push_str("}\n");
            }
        }
    }

    output.push_str("}\n");

    output
}

/// generate string of C++ code from a Brainf*ck string
pub fn brains(input: &str) -> String {
    let tokens = tokenize(input);

    let generated_code = generate(&tokens);

    generated_code
}
