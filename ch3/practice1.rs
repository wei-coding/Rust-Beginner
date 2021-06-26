use std::io;
use std::process;

fn main() {
    println!("input the temp to convert.");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error when input");
    let input: f32 = match input.trim().parse() {
        Ok(s) => s,
        Err(_) => {
            println!("Please enter a int or float.");
            process::exit(1);
        }
    };
    let mut convert_type = String::new();
    println!("input 0 for *C to *F, 1 for *F to *C.");
    io::stdin()
        .read_line(&mut convert_type)
        .expect("Error when input");
    let convert_type: i32 = match convert_type.trim().parse() {
        Ok(s) => s,
        Err(_) => {
            println!("Please enter a int");
            process::exit(1);
        }
    };
    if convert_type == 0 {
        let output: f32 = 9.0/5.0 * input + 32.0;
        println!("{:.2}*C is {:.2}*F", input, output);
    }else if convert_type == 1 {
        let output: f32 = 5.0/9.0 * input - 32.0;
        println!("{:.2}*F is {:.2}*C", input, output);
    }else{
        println!("bad convert type.");
    }
}