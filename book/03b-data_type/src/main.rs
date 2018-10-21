fn main() {
    println!("Scalar type: integer, float, boolean, character");
    println!("Unsigned Integers: u8, u16, u32, u64, u128, usize");
    println!("Signed Integers: :i8, i16, i32, i64, i128, isize");
    println!("Integer as Decimal: {}", 98_222);
    println!("Integer as Hex: {}", 0xff);
    println!("Integer as Octal: {}", 0o77);
    println!("Integer as Binary: {}", 0b1111_0000);
    println!("Integer as Byte: {}", b'A');
    // let mut x: u8 = 255;
    // Integer overflow
    // x = x + 1;
    // println!(
    //     "This will panic in debug and return 0 in release build: {}",
    //     x
    // );

    println!("Float: f32, f64 are IEEE-754 standard");
    let f1 = 2.0;
    println!("By default floats are f64 on 64 bit arch: {}", f1);
    let f2: f32 = 4.2;
    println!("Specifically setting the type to f32: {}", f2);

    println!("Numerical operations");
    println!("Sum of 5 and 10: {}", 5 + 10);
    println!("Difference of 5 and 10: {}", 5 - 10);
    println!("Product of 5 and 10: {}", 5 * 10);
    println!("Quotient of 5 and 10: {}", 5 / 10);
    println!("Remainder of 5 and 10: {}", 5 % 10);

    println!("Boolean types are true and false and are one byte in size");
    println!("Truthy: {}", true);
    println!("Falsy: {}", false);

    println!("Character type represent unicode scalar values");
    println!("Char can be an emoji: {}", '😻');
    println!("Char can be an accented letter: {}", 'ℤ');

    println!("Tuples don't need to be same type and cannot change in size");
    println!("Tuples is a compound type");
    let tup: (i32, f64, char) = (1, 2.0, 'a');
    println!(
        "need to use debug formatter to print a tuple to stdout {:?}",
        tup
    );
    println!(
        "We can access a tuple item using the dot operator by their index: {}",
        tup.2
    );
    let (a, b, c) = tup;
    println!(
        "Pattern matching to destructure the value of a tuple: {} {} {}",
        a, b, c
    );

    println!("Arrays is a collection of homogenous type");
    println!("Arrays are same length like tuples");
    println!("Arrays are useful when we want data on the stack instead of the heap");
    let days_of_week = ['m', 't', 'w', 't', 'f', 's', 's'];
    println!(
        "Use arrays when you know the values won't change: {:?}",
        days_of_week
    );
    let arr: [i32; 4] = [1, 2, 3, 4];
    println!("We can specifiy the type and size of the array: {:?}", arr);
    println!("Array is a single chunk of memory on the stack and value can be accessed using the index of the item");
    println!("First item of arr: {}", arr[0]);
}
