fn main() {
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    
    // loop with labels

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");


    let mut number = 10;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    let arr = [20, 40, 60, 80, 100];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", arr[index]);
        index += 1;
    }

    println!("");
    for ele in arr {
        println!("The element is: {ele}");
    }

    for number in (1..70).rev(){
        println!("{number}");
    }
}