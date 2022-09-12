# brainhug

[![Build Status](https://travis-ci.com/sn99/brainhug.svg?branch=master)](https://travis-ci.com/sn99/brainhug)
[![Build status](https://ci.appveyor.com/api/projects/status/23dcr0k5u244qd3e?svg=true)](https://ci.appveyor.com/project/sn99/brainhug)
[![Crates.io Download](https://img.shields.io/crates/d/brainhug.svg)](https://crates.io/crates/brainhug)
[![crate](https://img.shields.io/crates/v/brainhug.svg)](https://crates.io/crates/brainhug)
[![Documentation](https://docs.rs/brainhug/badge.svg)](https://docs.rs/brainhug)

`brainhug` is a crate that is used to interpret brainf*ck code to any other language

#### Current languages covered are :

- [x] C
- [x] C++
- [x] C#
- [x] Python
- [x] Golang
- [x] Haskell
- [x] Lua
- [X] JavaScript

#### Why the name brainhug ?

Inspired from [link](https://lists.freedesktop.org/archives/dri-devel/2018-November/198581.html)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
brainhug = "*"
```

and this to your crate root:

```rust
extern crate brainhug;

use brainhug::Lang;
```

## Contributing

Read [contributing](CONTRIBUTING.md) for Details

## Example

```rust
extern crate brainhug;

use brainhug::Lang;

fn main() {
    // will print `Hello World!`
    let input = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    // `brainhug::generate` will generate a `String`
    println!("{}", brainhug::generate(Lang::C, input));
}
```

#### It will produce

```c
#include "stdio.h"

int main() {
    char tape[20000] = {0};
    char *ptr = tape;
        *ptr += 8;
        while (*ptr) {
                ptr += 1;
                *ptr += 4;
                while (*ptr) {
                        ptr += 1;
                        *ptr += 2;
                        ptr += 1;
                        *ptr += 3;
                        ptr += 1;
                        *ptr += 3;
                        ptr += 1;
                        *ptr += 1;
                        ptr -= 4;
                        *ptr -= 1;
                }
                ptr += 1;
                *ptr += 1;
                ptr += 1;
                *ptr += 1;
                ptr += 1;
                *ptr -= 1;
                ptr += 2;
                *ptr += 1;
                while (*ptr) {
                        ptr -= 1;
                }
                ptr -= 1;
                *ptr -= 1;
        }
        ptr += 2;
        putchar(*ptr);
        ptr += 1;
        *ptr -= 3;
        putchar(*ptr);
        *ptr += 7;
        putchar(*ptr);
        putchar(*ptr);
        *ptr += 3;
        putchar(*ptr);
        ptr += 2;
        putchar(*ptr);
        ptr -= 1;
        *ptr -= 1;
        putchar(*ptr);
        ptr -= 1;
        putchar(*ptr);
        *ptr += 3;
        putchar(*ptr);
        *ptr -= 6;
        putchar(*ptr);
        *ptr -= 8;
        putchar(*ptr);
        ptr += 2;
        *ptr += 1;
        putchar(*ptr);
        ptr += 1;
        *ptr += 2;
        putchar(*ptr);
}
```

## License

Licensed under

* MIT license ([LICENSE.md](LICENSE.md) or http://opensource.org/licenses/MIT)
