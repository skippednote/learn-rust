#![allow(unused_variables)]

fn main() {
    // Scope of the variable y is upto end of the main function
    let y = "hello";
    {
        // scope of the x variable is upto the end of curly brace.
        let x = "world";
    }
    // x won't be availbale in this scope.
    // Rust calls drop on x once the scope closes.
    // This is similar to other languages like JavaScript.
    // println!("{}", x);

    // &str is immutable and is stored on the stack.
    // This can't be used in cases where the size is unknown.
    // This is why we have a String type.
    // converting &str to String
    // String::from asks the OS at runtime to allocate memory on the heap.
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{}", s);

    // since the value is of know size on stack, it's easy to clone the value
    // for stack values we don't need to explicitly call clone.
    // All Scalar types are Copy types. Tuples only they have other scalar values.
    let a = "cat";
    let b = a;

    println!("{} {}", a, b);

    let s1 = String::from("s1");
    let s2 = s1;
    // s1 will be moved and the won't need to be gc'ed.
    // s2 = s1 does a shallow clone where by copying pointer, length, and capacity of the original
    // value.
    //println!("{}", s1);
    // To copy the data and not just the pointer. We can use the clone method to deep copy.
    let s3 = s2.clone();
    // The value doesn't get moved incase of clone so we can print both s2 and s3.
    // clone is an expensive operation compared to move.
    println!("{} {}", s2, s3);

    let f = String::from("f");
    f1(f);
    // The value of f which is a value in the stack pointing to the heap is moved when passed to an
    // function.
    // println!("{}", f);

    let g = 42;
    f2(g);
    // Similar to how scalars variables work. The i32 function argument is copied.
    // The value will be dropped once the scope ends i.e main fn ends.
    println!("{}", g);

    // Will be dropped at end of scope.
    let s1 = give_ownership();
    // Is moved to when passed to take_and_give
    let s2 = String::from("Hello");
    // Will be dropped at end of scope
    let s3 = take_and_give(s2);
    println!("{}", s3);

    let name = String::from("Bassam Ismail");
    let (l, name) = cal_length(name);
    println!("The length of string: {} is {}", name, l);
}

fn f1(val: String) {
    println!("{}", val);
}

fn f2(val: i32) {
    println!("{}", val);
}

fn give_ownership() -> String {
    String::from("give")
}

// taking and returning ownership isn't the best solutions.
// we can instead use references.
fn take_and_give(val: String) -> String {
    val
}

fn cal_length(val: String) -> (usize, String) {
    (val.len(), val)
}
