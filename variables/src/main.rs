use std::io;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    let r = 10;
    let r = r + 59;
    {
        let r = r * 2;
        println!("The value of r in the inner scope is: {r}");
    }

    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");
    println!("The value of r is: {r}");

    let spaces = "    ";
    // spaces = spaces.len();
    let spaces = spaces.len();

    println!("{spaces}");

    let num = b'a';
    println!("{num}");

    let f_num = 69.69;
    let f_num_small: f32 = 0.69;

    println!("{f_num} & {f_num_small}");

    // tuple
    let tup = (10, 5.5, 2);

    let (a, b, c) = tup;
    println!("The value of a is: {a}, b is: {b} and c is: {c}");

    println!("{}", tup.0);

    // let empty_tup = ();

    let mix_data_type_tup = (69, 69.69, "sixty-nine", '6', b'9');

    println!(
        "{},\n{},\n{},\n{},\n{},\n",
        mix_data_type_tup.0,
        mix_data_type_tup.1,
        mix_data_type_tup.2,
        mix_data_type_tup.3,
        mix_data_type_tup.4
    );

    let _arr = [10, 20, 30, 40];
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [69; 3];
    println!("{}", b[0]);
    println!("{}", b[1]);

    let arr_to_guess_from = [1, 2, 3, 4, 5, 6];
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let ele = arr_to_guess_from[index];

    println!("The value of the element at index {index} is: {ele}");
}
