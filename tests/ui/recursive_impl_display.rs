use std::fmt::{Display, Formatter, Result};

struct A(String);

impl Display for A {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let str_repr = self.to_string();
        write!(f, "{}", str_repr)
    }
}

struct B(String);

impl Display for B {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let str_repr = &self.0;
        write!(f, "{}", str_repr)
    }
}

fn main() {
    let a: A = A("stack overflow".to_string());
    println!("{}", a.0);
    let b: B = B("correct".to_string());
    println!("{}", b);
}
