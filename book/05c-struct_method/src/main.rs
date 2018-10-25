struct Rectangle {
    width: u32,
    height: u32,
}

// We can use the implementation block to add methods to a struct/enum/trait object
// these methods can be called using dot syntax on the struct instance.
impl Rectangle {
    // functions inside impl that take self as the first parameter are known as methods
    // self can be passed a reference, mutable reference or pass ownership
    // we use &self if we can to use the values from struct
    // we use &mut self if we want to change the values of struct
    // we use self if we want to take self and return something else. This means self can't be used again.
    fn area(&self) -> u32 {
        // since the method takes in &self, we don't need to keep using &self inside the method.
        // We can simply use self.
        // This rust feature is called automatic referencing and derefrencing.
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // functions inside impl that don't self as a parameter are know as associated functions.
    // these functions are associated with the struct rather than the instance of a struct.
    // these are called using the :: operator.
    // Associated functions are usually used as constructors.

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// there can be multiple impl blocks for a struct.
impl Rectangle {
    fn lol(&self) {
        println!("lololol")
    }
}

fn main() {
    let r1 = Rectangle {
        width: 20,
        height: 30,
    };
    let r2 = Rectangle {
        width: 10,
        height: 20,
    };
    println!("The area of rectangle r1 is: {}", r1.area());
    println!("r1 can hold r2: {}", r1.can_hold(&r2));
    println!("r2 can hold r1: {}", r2.can_hold(&r1));

    let r3 = Rectangle::square(23);
    println!("The r3 width is {} and height is {}", r3.width, r3.height);
    r3.lol();
}
