extern crate brainhug;

use brainhug::Lang::C;

fn main() {
    let k = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    println!("{}", brainhug::generate(C, k));
}