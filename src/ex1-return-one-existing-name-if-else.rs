// ERROR RESULTS FROM:

fn foo<'a>(x: &'a i32, y: &i32) -> &'a i32 {
    if x > y { x } else { y }
}

fn main() { }
