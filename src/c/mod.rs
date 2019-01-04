use super::tokenize;
use super::Token;

fn generate(tokens: &[Token]) -> String {
    let mut output = String::from(include_str!("preface.c"));
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

pub fn brains(input: &str) -> String {
    let tokens = tokenize(input);

    let generated_code = generate(&tokens);

    generated_code
}
