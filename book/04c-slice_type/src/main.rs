fn main() {
    let mut name = String::from("Bassam Ismail");
    let fw = first_word(&name);
    println!("{}", fw);
    // Here we are clearing the string to "".
    name.clear();
    // Now we have a reference to the index of first word but the word is cleared which makes the
    // reference meaningless.
    println!("{} {}", name, fw);

    // We can address the problem of keeping track of index whoes underlying data can change using
    // String slices.
    // String slice stores a pointer to the starting index and the length of the slice.
    let s = String::from("Baheej Bassam");
    // This gets a reference of a slice of s from 0 index to (including) item at index 5.
    // .. is the range syntax
    // a..b is range starting from a to but not including b.
    // a..=b is range starting from a to including b.
    let b1 = &s[0..=5];
    // This gets a reference of a slice of s from 7 index to (including) item at index 12.
    let b2 = &s[7..=12];
    // Here we are getting from index 7 to end of string s
    let b2a = &s[7..];

    // This gets a string slice of the original string.
    let b3 = &s[..];
    println!("{}\n{}\n{}\n{}", b1, b2, b2a, b3);

    let b4 = &s[0..6];
    println!("From index 0 to but not including index 6: {}", b4);

    let b5 = &s[7..13];
    println!("From index 7 to but not including index 13: {}", b5);

    let b6 = &s[..6];
    println!(
        "If we drop the first index, the string slice would start from index 0: {}",
        b6
    );

    // Rewriting first_word fn using string slices
    // This implementation stays in sync with the original string.
    // If we clear the string then the return value will be an empty string and if we modify it, it
    // will find the correct first word.
    let name = String::from("Bassam Ismail");
    let fw2 = first_word_slice(&name);
    println!("{}", fw2);

    // String literals
    // s is &str. A slice pointing to specific point of the binary.
    // &str are immuatable variables and references.
    let s = String::from("Hello, World!");

    // Unlike first_word_slice here we take a &str and return &str. This makes the API a lot more
    // fluid.
    // We convert String to &str by creating a slice of the whole String.
    // String slice stores the reference to the first index and the len of the String.
    first_word_slice_in(&s[..]);

    // Slices can be made up of arrays too.
    let arr = [1, 2, 3, 4, 5];
    // Here a is array slice. It is a reference to the first element of the arr and it's len.
    // type of a is &[i32]
    let a = &arr[..];
    println!("{:?}", a);
}

fn first_word(name: &String) -> usize {
    // Convert String to an array of bytes.
    let bytes = name.as_bytes();

    // We convert the array to an iterator and then to a tuple with index and ref to value using
    // enumerate method.
    for (i, &byte) in bytes.iter().enumerate() {
        // If we find an item with space we return it's index
        if byte == b' ' {
            return i;
        }
    }

    // Else we return the length of whole string considering it's one word.
    name.len()
}

// Instead of usize(index or len) we are returning a slice of the first word or the whole word.
fn first_word_slice(name: &String) -> &str {
    let bytes = name.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            // Starting from index 0 upto the first space char.
            return &name[..i];
        }
    }

    // if no space is found, we will return the whole word.
    &name[..]
}

fn first_word_slice_in(name: &str) -> &str {
    let bytes = name.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            // Starting from index 0 upto the first space char.
            return &name[..i];
        }
    }

    // if no space is found, we will return the whole word.
    &name[..]
}
