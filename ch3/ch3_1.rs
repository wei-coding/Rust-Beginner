fn main() {
    // example for mutable
    // let x = 5;
    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);

    // example for shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x = {}", x);

    let spaces = "    ";
    let spaces = spaces.len();

    println!("num of spaces: {}", spaces);

    // ilegel syntax
    // let mut spaces = "    ";
    // spaces = spaces.len();
    // spaces.len() has return type usize, not &str


}
