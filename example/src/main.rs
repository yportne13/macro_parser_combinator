use macro_parser_combinator::*;

bnf!(a := "ab" << "c");
bnf!(b := "de" << "f");

bnf!(abc := a * b);
//bnf!(abc := "abc" * b);

fn main() {
    let x = "abcdef";
    x.get(0..x.len()).unwrap();
    println!("{:?}", abc!().run("abcdef"));
    //println!("Hello, world!");
}
