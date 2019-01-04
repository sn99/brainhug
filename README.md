# brainhug

[![Build Status](https://travis-ci.com/sn99/brainhug.svg?branch=master)](https://travis-ci.com/sn99/brainhug)
[![crate](https://img.shields.io/badge/crates.io-1.0-orange.svg)](https://crates.io/crates/brainhug)
[![documentation](https://img.shields.io/badge/docs-1.0-blue.svg)](https://docs.rs/brainhug)

`brainhug` is a crate that is used to interpret brainf*ck code to any other language

#### Current languages covered are :
- [x] C

##### Why the name brainhug ?
Inspired from [link](https://lists.freedesktop.org/archives/dri-devel/2018-November/198581.html)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
brainhug = "0.2.0"
```

and this to your crate root:

```rust
extern crate brainhug;
use brainhug::Lang;
```

## Example

```rust
extern crate brainhug;
use brainhug::Lang;

fn main() {
    // will print `Hello, World!`
    let input = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    println!("{}", brainhug::generate(Lang::C, input));
}
```

## License

Licensed under

 * MIT license ([LICENSE.md](LICENSE.md) or http://opensource.org/licenses/MIT)
