fn main() {
    let number: i32 = 42;

    // if expression runs based on the condition met.
    // It can have multiple branches.
    // The condition should always result in a boolean.
    // The following line works in most of the languages but it isn't a valid condition in rust.
    // if number {
    if number > 42 {
        println!("Too big");
    // else if can be used to check a additional case apart from if.
    // there can be multiple else if but single if and else branches.
    // once the condition is met the program will move out of the if block even if there are
    // multiple statisfied branches.
    // We should, however, avoid too many else if.
    } else if number < 42 {
        println!("Too small");
    } else {
        println!("We'll pin ya!")
    }

    // Since if is an expression we can assign it's value to a let
    // Value of name would be return value of if/else
    // return value of both if and else block should be the same.
    let name = if 5 > 0 {
        println!("Is greater");
        5
    } else {
        42
    };
    println!("{:?}", name);

    // To run code more than once rust provides loops: loop, while, for.
    let mut i = 0;
    // loop is an expression and when it exists it can return a value.
    let val = loop {
        i = i + 1;
        if i % 2 == 0 {
            println!("{} =>", i);
        } else {
            continue;
        }

        if i == 42 {
            break i;
        }
    };
    assert_eq!(val, 42);

    // While runs till the condition is met. Then it breaks out.
    // We can build a while using loop and if else but it's built into rust
    let mut counter = 10;
    while counter > 0 {
        println!("Counter is going: {}", counter);
        counter = counter - 1;
    }

    // While loop isn't the best option for example when looping over an array as the length can be
    // incorrect and the calcutions take time to compute.
    // For loop is a better alternative in such case.
    // It is the most used loop because of it's safety and consise syntax.
    let a = [1, 2, 3, 4, 5];
    for i in a.iter() {
        println!("a => {}", i);
    }
    // To run a piece of code certain number of times we could use ranges with for loop.
    for number in (1..4).rev() {
        println!("Ranging: {}", number);
    }
}
