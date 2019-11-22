# brainhug

[![Build Status](https://travis-ci.com/sn99/brainhug.svg?branch=master)](https://travis-ci.com/sn99/brainhug)
[![Build status](https://ci.appveyor.com/api/projects/status/23dcr0k5u244qd3e?svg=true)](https://ci.appveyor.com/project/sn99/brainhug)
[![Crates.io Download](https://img.shields.io/crates/d/brainhug.svg)](https://crates.io/crates/brainhug)
[![crate](https://img.shields.io/crates/v/brainhug.svg)](https://crates.io/crates/brainhug)

`brainhug` is a crate that is used to interpret brainf*ck code to any other language

#### Current languages covered are :
- [x] C
- [x] C++
- [x] Python

#### Why the name brainhug ?
Inspired from [link](https://lists.freedesktop.org/archives/dri-devel/2018-November/198581.html)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
brainhug = "0.2.1"
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

    ++*ptr;
    ++*ptr;
    ++*ptr;
    ++*ptr;
    ++*ptr;
    ++*ptr;
    ++*ptr;
    ++*ptr;
    while (*ptr) {
        ++ptr;
        ++*ptr;
        ++*ptr;
        ++*ptr;
        ++*ptr;
        while (*ptr) {
            ++ptr;
            ++*ptr;
            ++*ptr;
            ++ptr;
            ++*ptr;
            ++*ptr;
            ++*ptr;
            ++ptr;
            ++*ptr;
            ++*ptr;
            ++*ptr;
            ++ptr;
            ++*ptr;
            --ptr;
            --ptr;
            --ptr;
            --ptr;
            --*ptr;
        }
        ++ptr;
        ++*ptr;
        ++ptr;
        ++*ptr;
        ++ptr;
        --*ptr;
        ++ptr;
        ++ptr;
        ++*ptr;
        while (*ptr) {
            --ptr;
        }
        --ptr;
        --*ptr;
    }
    ++ptr;
    ++ptr;
    putchar(*ptr);
    ++ptr;
    --*ptr;
    --*ptr;
    --*ptr;
    putchar(*ptr);
    ++*ptr;
    ++*ptr;
    ++*ptr;
    ++*ptr;
    ++*ptr;
    ++*ptr;
    ++*ptr;
    putchar(*ptr);
    putchar(*ptr);
    ++*ptr;
    ++*ptr;
    ++*ptr;
    putchar(*ptr);
    ++ptr;
    ++ptr;
    putchar(*ptr);
    --ptr;
    --*ptr;
    putchar(*ptr);
    --ptr;
    putchar(*ptr);
    ++*ptr;
    ++*ptr;
    ++*ptr;
    putchar(*ptr);
    --*ptr;
    --*ptr;
    --*ptr;
    --*ptr;
    --*ptr;
    --*ptr;
    putchar(*ptr);
    --*ptr;
    --*ptr;
    --*ptr;
    --*ptr;
    --*ptr;
    --*ptr;
    --*ptr;
    --*ptr;
    putchar(*ptr);
    ++ptr;
    ++ptr;
    ++*ptr;
    putchar(*ptr);
    ++ptr;
    ++*ptr;
    ++*ptr;
    putchar(*ptr);
}
```

## License

Licensed under

 * MIT license ([LICENSE.md](LICENSE.md) or http://opensource.org/licenses/MIT)
