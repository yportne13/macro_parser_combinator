use macro_parser_combinator::*;

bnf!(lit_temp := "null" | "true" | "false");
bnf!(lit := whitespace >> lit_temp);
bnf!(array := [lit]);

//bnf!(a := "ab" << "c");
//bnf!(b := "de" << "f");
bnf!(ztoa := r"[a-z]*");

//bnf!(abc := a * b);
//bnf!(abc := "abc" * b);

fn main() {
    //println!("{:?}", abc!().run("abcdef"));

    println!("{:?}", lit!().run("null"));
    println!("{:?}", lit!().run("true"));
    println!("{:?}", lit!().run("false"));
    println!("{:?}", array!().run("[false]"));
    
    
    
    println!("{:?}", ztoa!().run("json123abc"));

    let parser = token!(r#"r""#) >> regex!(r#"[^"]*"#);
    println!("{:?}", parser.run(r#"r"[a-z]*""#))
}
