#[macro_use]
extern crate criterion;

use brainhug::Lang;
use criterion::Criterion;

fn bench(c: &mut Criterion) {
    c.bench_function("Lua", |b| {
        let brainfuck_string = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        b.iter(|| brainhug::generate(Lang::Lua, brainfuck_string))
    });
    c.bench_function("C++", |b| {
        let brainfuck_string = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        b.iter(|| brainhug::generate(Lang::Cpp, brainfuck_string))
    });
    c.bench_function("C", |b| {
        let brainfuck_string = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        b.iter(|| brainhug::generate(Lang::C, brainfuck_string))
    });
    c.bench_function("C#", |b| {
        let brainfuck_string = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        b.iter(|| brainhug::generate(Lang::CSharp, brainfuck_string))
    });
    c.bench_function("Python", |b| {
        let brainfuck_string = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        b.iter(|| brainhug::generate(Lang::Python, brainfuck_string))
    });
    c.bench_function("Golang", |b| {
        let brainfuck_string = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        b.iter(|| brainhug::generate(Lang::GoLang, brainfuck_string))
    });
    c.bench_function("Haskell", |b| {
        let brainfuck_string = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        b.iter(|| brainhug::generate(Lang::Haskell, brainfuck_string))
    });
    c.bench_function("JavaScript", |b| {
        let brainfuck_string = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        b.iter(|| brainhug::generate(Lang::JavaScript, brainfuck_string))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
