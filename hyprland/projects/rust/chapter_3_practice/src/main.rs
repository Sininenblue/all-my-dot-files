use std::ops::Range;

fn main() {
    println!("Hello, world!");

    // farenheit to celsius
    let cel = f_to_c(36.0);
    println!("f to c {cel}");
    // generate nth fibonacci number
    let fib = fibonacci_generator(7);
    println!("fib is {fib}");

    // lyrcs to the twelve days of christmas
}

fn f_to_c(temperature: f64) -> f64 {
    (temperature - 32.0) * (5.0 / 9.0)
}

fn fibonacci_generator(number: i32) -> i32 {
    let mut value_before = 0;
    let mut temp = 0;
    let mut current_fibonacci = 1;
    let range = Range {
        start: 0,
        end: number,
    };
    for iteration in range {
        prntln!("iteration");
        temp = current_fibonacci + value_before;
        current_fibonacci = value_before;
        value_before = temp;
    }
    temp
}
