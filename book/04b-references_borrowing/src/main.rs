#![allow(unused_variables)]
fn main() {
    let s1 = String::from("hello");
    // We are passing reference of s1 instead of moving it.
    // value will be dropped as soon as it goes out of the scope.
    cal_len(&s1);

    // let s2 = String::from("world");
    // This will panic as we are trying to make change to an immutable reference.
    // imm_borrow(&s2);

    // Just like variables, reference are immutable by default.
    // To make changes to them we have to make reference mutable.
    let mut s3 = String::from("world");
    // To make changes to a reference we have to pass a mutable reference.
    mut_borrow(&mut s3);
    // One of the restrictions of mutable references is that we can only have one reference per
    // scope.
    // This prevents data race. For example, when two pointers are trying to write to the same
    // reference.
    {
        // we can create a new mutable reference by creating a new scope.
        let rz = &mut s3;
        println!("{:?}", rz);
    }
    // rz is out of scope here.
    println!("New Scope {}", s3);

    let mut h = "hello";
    let h1 = &h;
    let h2 = &h;
    // We can't have immutable and mutable borrow in the same scope.
    // let h3 = &mut h;
    // This is because we don't immutable data suddenly to be changed.

    let a = dangle();
    println!("{}", a);
}

// We receive the reference of s1 as &String.
// Passing a reference to a function is also known as borrowing.
fn cal_len(s: &String) -> usize {
    // When s goes out of scope here nothing happens as s hasn't been moved by cal_len
    s.len()
}

// borrows are immutable by default. We need to pass and take an mutable borrow to make the
// changes.
// fn imm_borrow(s: &String) {
//     s.push_str("Hoi")
// }

fn mut_borrow(s: &mut String) {
    s.push_str(" Hoi")
}

// Since s is deallocated once the dangle scope end the function is throwing an error.
// Unlike other languages, there can't be references to memory that are cleared.
// fn dangle() -> &String {
//     let s = String::from("dangle");
//     &s
// }
// To address this, we can return a String instead. This way we are moving the ownership of the s
// variable.
fn dangle() -> String {
    let s = String::from("dangle");
    s
}
