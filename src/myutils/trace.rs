use std::fmt::Debug;

pub fn trace<A: Debug>(s: &str, a: A) -> A {
    println!("{s}: {a:?}");
    a
}
