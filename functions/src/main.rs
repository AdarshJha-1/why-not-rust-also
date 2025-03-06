fn main() {
    println!("Hello, world!");
    another_function(69, 'A');

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let five_value = five();
    println!("{five_value}");
}

fn another_function(p1: i32, p2: char) {
    println!("Another function.");
    println!("The value of p1 is: {p1}");
    println!("The value of p2 is: {p2}");
}


// Comments yaya!!
fn five() -> i32 {
    5
}