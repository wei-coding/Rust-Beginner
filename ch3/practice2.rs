use std::io;
use std::process;

fn main() {
    println!("input n, to find the n th fibonacci number.");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error when reading");
    let input: i32 = match input.trim().parse() {
        Ok(s) => s,
        Err(_) => {
            println!("Please enter a int.");
            process::exit(1);
        }
    };
    println!("the {}th fibonacci number is {}", input, fib(input));
}

fn fib(n: i32) -> i64 {
    if n == 1 || n == 2 {
        return 1;
    }
    let r = fib(n - 1) + fib(n - 2);
    return r;
}