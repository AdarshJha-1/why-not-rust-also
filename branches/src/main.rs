fn main() {
    let number = 3;

    if number < 5 {
        println!("condition wa true");
    } else {
        println!("condition was false");
    }

    let bool_true_or_false = true;
    if bool_true_or_false {
        println!("running...");
    }

    let integer_val_for_condition = 20;
    if integer_val_for_condition != 0 {
        println!("running...");
    }

    // if, else if, else
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;

    let number = if condition {5} else {6};
    println!("The value of number is: {number}")
}