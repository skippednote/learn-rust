// Struct are similar to tuples where they group related type of data.
// Each piece of data in struct has a name unlike tuples.
// struct name should be in Camel Case and the fields should be lowercase.
struct User {
    name: String,
    age: i32,
    active: bool,
    // if we are using referenced data we have to specify life time on the struct
    // job: &str,
}

// Tuple struct are similar to structs but their fields don't have names.
// This is useful when we want to give tuples name to distinguish.
// Color and Point have same internals where they take 3 i32 but when creating them or passing to
// function we are able to distinguish. A function that takes Color struct won't take Point even
// though both have 3 i32 integers.
// Value of tuple struct is accessed by there index
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like struct are useful when we don't have any data but want to implement some trait on
// them.
struct Bassam {}

fn main() {
    // Order of the fields of the struct doesn't matter.
    // However all the fields of the struct should be populated.
    let mut u1 = User {
        age: 27,
        name: String::from("Bassam"),
        active: true,
    };

    // Use dot notation to get a value from a struct.
    println!("{:?}", u1.name);

    // If the struct is mutable, then we can change the value of a field.
    // The whole struct is mutable, specific fields can't be.
    u1.age = 26;

    // Since struct is a expression it can be used in the last line of a function for return.
    create_user(String::from("bassam"), 27);

    // Struct update syntax makes it easy to use fields value for an other existing strut
    let u2 = User {
        name: String::from("Aliya"),
        ..u1
    };
    println!(
        "Name is update to: {}\nAge is same as u1: {}",
        u2.name, u2.age
    );

    let c = Color(0, 0, 255);
    let p = Point(100, 0, 200);
    println!("The value of Blue is: {}", c.2);
    only_point(p);
    // This won't work as only_point takes tuple struct of type Point.
    // only_point(c);
}

fn create_user(name: String, age: i32) -> User {
    // Since the field name and function parameters are the same we can use the
    // field init shorthand syntax.
    User {
        name,
        age,
        active: true,
    }
}

fn only_point(p: Point) {
    println!("{}", p.1)
}
