struct Ref<'a, T: 'a> {
    data: &'a T
}

fn foo<'a>(x: &mut Vec<Ref<'a, i32>>, y: Ref<i32>) {
//                         --         - consider changing the type of `y` to `Ref<'a, i32>`
//                         |
//                   lifetime from `x`
        x.push(y);
//      ^ only legal if `x` and `y` have same lifetime
}

fn main() { }
