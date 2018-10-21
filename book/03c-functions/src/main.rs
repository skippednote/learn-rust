fn main() {
    // Functions are snake case in rust.
    // Function can be defined after the function call.
    fn_one();
    fn_two(-1);
    fn_three("Bassam", 27);
    fn_four();
    fn_five();
    fn_six(27);
}

fn fn_one() {
    println!("Another Function");
}

// Type of the parameter needs to be specified.
fn fn_two(x: i32) {
    println!("The value of x is: {}", x);
}

// Function paramters are comma separated.
fn fn_three(x: &str, y: i32) {
    println!("Hello my name is {} and I'm {} years old", x, y);
}

fn fn_four() {
    // This is a statement as it doesn't return any value.
    // Function definition are also statements as they won't return anything unless called.
    let y = 42;

    // + is an expression as it return the value 42 + 1 which is used by println!
    println!("{}", y + 1);

    // An expression can be assigned to a statement.
    // {} creates a new scope.
    let z = {
        let x = 1;
        // We aren't adding a semicolon here because that converts it into an statement and that
        // won't return any value.
        x + 2
    };
    println!("{}", z);
}

// The return value is declared using the -> and type String in this case.
fn fn_five() -> String {
    String::from("String will be returned")
}

fn fn_six(age: i32) -> i32 {
    if age < 18 {
        // use return keyword to exit early.
        return 0;
    } else {
        return 1;
    }
}
