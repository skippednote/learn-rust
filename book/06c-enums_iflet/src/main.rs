fn main() {
    let s = Some(0u8);

    // here we are intersted only if value of s is 3. We don't care for any other value.
    // We are using match here but instead we can use a simpler alternative: if let.
    match s {
        Some(3) => println!("three"),
        _ => () // this is to make sure match is exhaustive to cover all the possible scenarios.
    }

    if let Some(3) = s { // pattern and expression are separted by =, not to be confused with assignment operator
        println!("three");
    } else { // this is similar to _ wildcard match in match expression
        println!("not sure");
    }
}
