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

fn generate(tokens: &[Token]) ->  String {
    let mut output = String::from(include_str!("preface.rs"));
    let mut indent = 1;

    for &token in tokens {
        match token {
            Add => {
                for _ in 0..indent {
                    output.push_str("\t");
                }
                output.push_str("tape[*ptr] += 1;\n");
            },
            Sub => {
                for _ in 0..indent {
                    output.push_str("\t");
                }
                output.push_str("tape[*ptr] -= 1;\n");
            },
            Right => {
                for _ in 0..indent {
                    output.push_str("\t");
                }
                output.push_str("*ptr += 1;\n");
            },
            Left => {
                for _ in 0..indent {
                    output.push_str("\t");
                }
                output.push_str("*ptr -= 1;\n");
            },
            Read => {
                let mut buf = String::new();
                for _ in 0..indent {
                    buf.push_str("\t");
                }
                output.push_str(&format!("{}let mut input = [0; 1];\n", buf));
                output.push_str(&format!("{}std::io::stdin().read_exact(&mut input).expect(\"failed to read stdin\");\n", buf));
                output.push_str(&format!("{}tape[*ptr] = input[0];\n", buf));
            },
            Write => {
                for _ in 0..indent {
                    output.push_str("\t");
                }
                output.push_str("print!(\"{}\", tape[*ptr] as char);\n");
            },
            BeginLoop => {
                indent += 1;

                for _ in 0..indent {
                    output.push_str("\t");
                }
                output.push_str("while tape[*ptr] != 0 {\n");
            },
            EndLoop => {
                indent -= 1;

                for _ in 0..indent {
                    output.push_str("\t");
                }
                output.push_str("}\n");
            },
        }
    }

    output.push_str("}");

    output
}

pub fn brains(input: &str) -> String {
    let tokens = tokenize(input);

    generate(&tokens)
}

#[test]
fn rust() {
    use super::*;

    println!("{}", brains("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.").to_string());
}