use std::io;


fn main() {
    println!("Enter temp in °F");
    let mut temp_f = String::new();
    io::stdin().read_line(&mut temp_f).expect("please give a valid input");
    let temp_f: f32 = temp_f.trim().parse().expect("error parsing to f32"); 

    let temp_c = convert_temp(temp_f);
    println!("{temp_f}°F is {:.2}°C", temp_c);

    let ans = generate_nth_fibo(5);
    println!("Fibonacci of 5 is: {ans}");
}

fn convert_temp(f: f32) -> f32 {
   (f - 32.0) * 5.0 / 9.0
}

fn generate_nth_fibo(n: u32) -> u32 {
    if n == 1 {
        return 1;
    } else if n== 0 {
        return 0;
    }
    return generate_nth_fibo(n - 1) + generate_nth_fibo(n - 2);
}