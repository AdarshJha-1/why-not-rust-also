fn main() {
    let mut s = String::from("Hello");
    //   s.push('C'); // push only character

    s.push_str(", World!");
    println!("{s}");

    /*
    let s1 = String::from("JAVA");
    let s2 = s1;

    println!("{s1} Sucks");
    println!("{s2} Sucks");
    */

    let mut s1 = String::from("First String");
    s1 = String::from("Second String");
    println!("{s1}");

    let s2 = s1.clone();
    println!("s1: {s1}, clone s2: {s2}");


    let x = 69;
    let y = x;

    println!("x = {x}, y = {y}");


    let str1 = String::from("L-L-L-L-Luffy-senpai!!");

    println!("Before calling take_ownership: {str1}");
//    take_ownership(str1);
    println!("After calling take_ownership: {str1}");

    let my_x = 69;
    println!("Before calling makes_copy: {my_x}");
    makes_copy(my_x);
    println!("After calling makes_copy: {my_x}");

    let str2 = give_ownership();
    println!("{str2}");

    let str3 = give_and_take_ownership(str1);
    println!("{str3}");


    let str4 = String::from("Some String");
    let (str5, str5_len) = calculate_str_len(str4);
    println!("{str5}, len: {str5_len}");
}

fn take_ownership(some_srting: String) {
    println!("some_srting");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn give_and_take_ownership(some_str: String) -> String {
    some_str
}


fn give_ownership() -> String {
    let my_str = String::from("iykyk");
    my_str
}


fn calculate_str_len(s: String) -> (String, usize) {
    let str_len = s.len();

    (s, str_len)
}
