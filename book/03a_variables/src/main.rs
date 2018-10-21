const MAX_VALUE: u32 = 100_000;

fn main() {
    println!("Constant in global scope: {}", MAX_VALUE);
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is mutated to: {}", x);

    let x = 42;
    println!("Shadowing x to a immutable value: {}", x);
}
