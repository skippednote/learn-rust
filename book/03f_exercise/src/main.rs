fn c_to_f(c: i32) -> i32 {
    (c * 9 / 5) + 32
}

fn f_to_c(f: i32) -> i32 {
    (f - 32) * 5 / 9
}

fn fib(i: u64) -> u64 {
    match i {
        1 | 2 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}

fn main() {
    println!("50c to Farenheit is: {}f", c_to_f(50));
    println!("50f to Celcius is: {}c", f_to_c(50));
    println!("Fibonnaci of 9 is: {}", fib(9));
    let song = "
        On the first day of Christmas, my true love gave to me
A partridge in a pear tree
On the second day of Christmas, my true love gave to me
Two turtle doves and a partridge in a pear tree
On the third day of Christmas, my true love gave to me three French hens
Two turtle doves and a partridge in a pear tree
On the fourth day of Christmas, my true love gave to me
Four calling birds, three French hens, two turtle doves
And a partridge in a pear tree
On the fifth day of Christmas, my true love gave to me
Five golden rings, four calling birds, three French hens
Two turtle doves and a partridge in a pear tree
On the sixth day of Christmas, my true love gave to me
Six geese a-layin', five golden rings, four calling birds
Three French hens, two turtle doves and a partridge in a pear tree
On the seventh day of Christmas, my true love gave to me
Seven swans a-swimmin', six geese a-layin', five golden rings
Four calling birds, three French hens, two turtle doves
And a partridge in a pear tree
On the eighth day of Christmas, my true love gave to me
Eight maids a-milkin', seven swans a-swimmin'
Six geese a-layin', five golden rings, four calling birds, three French hens
Two turtle doves and a partridge in a pear tree
On the ninth day of Christmas, my true love gave to me
Nine lords a-leapin', eight maids a-milkin', seven swans a-swimmin'
Six geese a-layin', five golden rings, four calling birds, three French hens
Two turtle doves and a partridge in a pear tree
On the tenth day of Christmas, my true love gave to me
Ten ladies dancin', nine lords a-leapin', eight maids a-milkin'
Seven swans a-swimmin', six geese a-layin', five golden rings
Four calling birds, three French hens
Two turtle doves and a partridge in a pear tree
On the eleventh day of Christmas, my true love gave to me
Eleven pipers pipin', ten ladies dancin', nine lords a-leapin'
Eight maids a-milkin', seven swans a-swimmin'
Six geese a-layin', five golden rings, four calling birds, three French hens
Two turtle doves and a partridge in a pear tree
On the twelfth day of Christmas, my true love gave to me
Twelve drummers drummin', eleven pipers pipin', ten ladies dancin'
Nine lords a-leapin', eight maids milkin', seven swans a-swimmin'
Six geese a-layin' five golden rings, four calling birds, three French hens
Two turtle doves and a partridge in a pear tree
        ";
    let song = song.split("\n");
    for line in song {
        println!("{}", line);
    }
}
