fn main() {
    let a = 1;
    let b = 2;
    println!("a + b = {}", add_with_lifetime(&a, &b));
}

fn add_with_lifetime<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}
