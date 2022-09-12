use super::tokenize;
use super::Token;
use super::Token::*;

fn generate(tokens: &[Token]) -> String {
    let mut output = String::from(include_str!("preface.cs"));
    let mut indent = 1;
    let mut aritms = 0;
    let mut arrows = 0;

    for &token in tokens {
        match token {
            Add => {
                if arrows != 0 {
                    for _ in 0..indent {
                        output.push('\t');
                    }
                    push_arrows(&mut output, arrows);
                    arrows = 0;
                }
                // Increment the value at the selected cell
                aritms += 1;
            }
            Sub => {
                if arrows != 0 {
                    for _ in 0..indent {
                        output.push('\t');
                    }
                    push_arrows(&mut output, arrows);
                    arrows = 0;
                }
                // Decrement the value at the selected cell
                aritms -= 1;
            }
            Right => {
                if aritms != 0 {
                    for _ in 0..indent {
                        output.push('\t');
                    }
                    push_aritms(&mut output, aritms);
                    aritms = 0;
                }
                // Change our selected cell to the next to the right
                arrows += 1;
            }
            Left => {
                if aritms != 0 {
                    for _ in 0..indent {
                        output.push('\t');
                    }
                    push_aritms(&mut output, aritms);
                    aritms = 0;
                }
                // Change our selected cell to the next to the left
                arrows -= 1;
            }
            others => {
                if aritms != 0 {
                    for _ in 0..indent {
                        output.push('\t');
                    }
                    push_aritms(&mut output, aritms);
                    aritms = 0;
                } else if arrows != 0 {
                    for _ in 0..indent {
                        output.push('\t');
                    }
                    push_arrows(&mut output, arrows);
                    arrows = 0;
                }
                match others {
                    EndLoop => {
                        indent -= 1;

                        for _ in 0..indent {
                            output.push('\t');
                        }
                        // Close a loop
                        output.push_str("}\n\n");
                    }
                    Read => {
                        for _ in 0..indent {
                            output.push('\t');
                        }
                        // Read a single character into the selected cell
                        output.push_str("tape[index] = Console.ReadKey().KeyChar;\n");
                    }
                    Write => {
                        for _ in 0..indent {
                            output.push('\t');
                        }
                        // Print the character at the selected cell
                        output.push_str("Console.Write(tape[index]);\n");
                    }
                    BeginLoop => {
                        indent += 1;

                        for _ in 0..(indent - 1) {
                            output.push('\t');
                        }
                        // Begin a loop at the current cell
                        output.push_str("while (tape[index] != 0)\n");

                        for _ in 0..(indent - 1) {
                            output.push('\t');
                        }
                        // Add a starting bracket at the next line
                        output.push_str("{\n");
                    }
                    _ => {}
                }
            }
        }
    }

    output.push_str("\t}\n}\n");

    output
}

fn push_aritms(out: &mut String, aritms: isize) {
    if aritms > 0 {
        out.push_str(format!("tape[index] += (char){};\n", aritms).as_str());
    } else {
        out.push_str(format!("tape[index] -= (char){};\n", -aritms).as_str());
    }
}

fn push_arrows(out: &mut String, arrows: isize) {
    if arrows > 0 {
        out.push_str(format!("index += {};\n", arrows).as_str());
    } else {
        out.push_str(format!("index -= {};\n", -arrows).as_str());
    }
}

/// generate string of C code from a Brainf*ck string
pub fn brains(input: &str) -> String {
    let tokens = tokenize(input);

    generate(&tokens)
}
