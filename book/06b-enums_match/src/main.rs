#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState) // here data can be bound to an enum variant
}

fn value_in_cents(coin: Coin) -> u32 {
    // we can use the match operator to compare values.
    // unlike the if expression the value can be of any type and not just boolean.
    // for match to work it has to be exhaustive and all scenarios need to be covered.
    // Once the coin variables matches one of the enum type the arm in the match return the value.
    match coin {
        // an arm can have multiple statements
        // but it should return the expected return type in this case a u32
        Coin::Penny => {
            println!("Pretty penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }
}

// we can get the value out of Option<T> using match operator using match.
// however we need to be exhaustive and check for the None case too.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => {
            println!("{}", i);
            Some(i)
        }
    }
}

fn simple_match(coin: Coin) -> i8 {
    // we can have a match that doesn't cover all the values
    // we can match specific values and for the rest we can use _ and return a single value.
    match coin {
        Coin::Penny => 1,
        Coin::Dime => 2,
        _ => 0 // here _ is a placeholder value.
    }
}

fn main() {
    let p1 = Coin::Penny;
    let p1r = value_in_cents(p1);
    println!("p1r is {}", p1r);

    let q1 = Coin::Quarter(UsState::Alaska);
    // we take a quarter and pass it alaska
    // now if we want to get the value of state, we can use match operator and pattern matching
    let q1r = value_in_cents(q1);
    println!("{}", q1r);

    let five = Some(5);
    let po = plus_one(five);
    println!("{:?}", po);
    let no = None;
    let po1 = plus_one(no);
    println!("{:?}", po1);

    let n = Coin::Nickel;
    let nr = simple_match(n);
    println!("{}", nr);
}
