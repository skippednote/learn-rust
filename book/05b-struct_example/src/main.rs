#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let w1 = 30;
    let h1 = 50;
    println!("The area of r1 is {}", area_vars(w1, h1));

    let r2 = (10, 23);
    println!("The area of r2 is {}", area_tuples(r2));

    let r3 = Rectangle {
        width: 33,
        height: 11,
    };
    println!("The area of r3 is {}", area_struct(&r3));
    // We won't be able to print r3 ro stdout as Rectangle struct doesn't implement the
    // std::fmt::Display trait.
    // This doesn't throw errors for scalar values because there is only one way to show them (eg 1
    // will always be 1).
    // println!("The rect r3: {}", r3);
    // We can use the debug formatter which is useful during development/debugging.
    // But to use it we have to add an annotation because rust doesn't include it by default.
    // This will print the whole output of println! in a single line.
    println!("The rect r3: {:?}", r3);
    // If the struct is big it can get difficult to read it all in one line.
    // In these cases we can use {:#?} instead of {:?}
    println!("The rect r3: {:#?}", r3);
    // Rust provides a lot of traits that can be added as annotation to add new feature.
    // These traits are called Derivable Traits.
}

// Both variables width and height are related to the rectangle but are two distinct variables. It
// would be better if we can group them.
fn area_vars(width: u32, height: u32) -> u32 {
    width * height
}

// With tubles the values are grouped together however, we get the variables by index and not by
// name which can be confusing and lead to incorrect results.
fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Here we have moved data to a struct with named fields.
// We should borrow a reference of a struct rather than take ownership of it.
// This way main retains the ownership of r3 and can continue using it.
fn area_struct(r: &Rectangle) -> u32 {
    r.width * r.height
}
