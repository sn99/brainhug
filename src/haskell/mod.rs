use super::tokenize;
use super::Token;
use super::Token::*;

fn generate(tokens: &[Token]) -> String {
    let mut output = String::from(include_str!("preface.hs"));
    let mut indent = 1;

    for &token in tokens {
        match token {
            Add => {
                for _ in 0..indent {
                    output.push_str("    ");
                }
                // Increment the value at the selected cell
                output.push_str(">=> inc\n");
            }

            Sub => {
                for _ in 0..indent {
                    output.push_str("    ");
                }
                // Decrement the value at the selected cell
                output.push_str(">=> dec\n");
            }

            Right => {
                for _ in 0..indent {
                    output.push_str("    ");
                }
                // Change our selected cell to the next to the right
                output.push_str(">=> right\n");
            }

            Left => {
                for _ in 0..indent {
                    output.push_str("    ");
                }
                // Change our selected cell to the next to the left
                output.push_str(">=> left\n");
            }

            Read => {
                for _ in 0..indent {
                    output.push_str("    ");
                }
                // Read a single character into the selected cell
                output.push_str(">=> getC\n");
            }

            Write => {
                for _ in 0..indent {
                    output.push_str("    ");
                }
                // Print the character at the selected cell
                output.push_str(">=> putC\n");
            }

            BeginLoop => {
                for _ in 0..indent {
                    output.push_str("    ");
                }
                indent += 1;
                // Begin a loop at the current cell
                output.push_str(">=> while (return\n");
            }

            EndLoop => {
                indent -= 1;
                for _ in 0..indent {
                    output.push_str("    ");
                }
                // Close a loop
                output.push_str(")\n");
            }
        }
    }

    output.push('\n');

    output
}

/// generate string of Haskell code from a Brainf*ck string
pub fn brains(input: &str) -> String {
    let tokens = tokenize(input);

    generate(&tokens)
}
