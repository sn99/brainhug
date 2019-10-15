#[macro_use]
extern crate criterion;

use brainhug::Lang;
use criterion::Criterion;

fn bench(c: &mut Criterion) {
    c.bench_function("Rust", |b| {
        let brainfuck_string = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        b.iter(|| brainhug::generate(Lang::Rust, brainfuck_string))
    });
    c.bench_function("C++", |b| {
        let brainfuck_string = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        b.iter(|| brainhug::generate(Lang::Cpp, brainfuck_string))
    });
    c.bench_function("C", |b| {
        let brainfuck_string = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        b.iter(|| brainhug::generate(Lang::C, brainfuck_string))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
