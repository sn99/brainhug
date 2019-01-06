The crate has been modulated for easier addition of new languages

### Steps ---

**1.** Create a new folder for your language in [src](https://github.com/sn99/brainhug/tree/master/src)

**2.** Make the format same as 
```rust
use super::tokenize;
use super::Token::*;
use super::Token;

fn generate(tokens: &[Token]) -> String {
    let mut output = String::from(include_str!("preface.<your language extension>"));
    let mut indent = 1;

```
-------------------------------------------------------------------------------------------------
```rust
// The code has been written for C but you have to change it for your language

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
```
------------------------------------------------------------------------------------------------------------------
```rust
/// generate string of <your language> code from a Brainf*ck string
pub fn brains(input: &str) -> String {
    let tokens = tokenize(input);

    let generated_code = generate(&tokens);

    generated_code
}
```
See [file](https://github.com/sn99/brainhug/tree/master/src/c) for a better understanding

**3.** Next edit [lib.rs](https://github.com/sn99/brainhug/blob/master/src/lib.rs) to contain

```rust
pub mod c;
pub mod cpp;
pub mod <your language folder>;
```
-----------------------------------------------------------------------------------------------------------------------
```rust
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
```
--------------------------------------------------------------------------------------------------------------------------
```rust
/// languages covered by the crate
/// Use `Lang::` to  specify a language to be used
pub enum Lang {
    C,                   // C language
    Cpp,                 // C++ language
    <Your language name> // <Your language name>
}
```
-----------------------------------------------------------------------------------------------------------------------------
```rust
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
        <Your language name> => {
            use crate::<your language folder>::brains;
            brains(input).to_string()
        },
        _ => {
            input.to_string()
        }
    }
}
```

*** Finally build to check

**Keep your language folder name in lower_case**
