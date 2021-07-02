fn main() {

    // int
    let guess: u32 = "42".parse().expect("Not a number!");
    // invalid example
    // let guess = "42".parse().expect("Not a number!");

    // int have below type: i/u8~128, i/usize
    // representation
    // 10: 98_222
    // 16: 0xff
    // 8: 0o77
    // 2: 0b1111_0001
    // byte(u8): b'A'
    println!("guess = {}", guess);

    // float
    let x = 2.0; // f64, double
    let y: f32 = 3.0; //f32, single

    println!("x = {}, y = {}", x, y);

    // boolean
    let t = true;
    let f: bool = false;
    println!("t = {}, f = {}", t, f);

    // character
    let c = 'z';
    let z = 'Z';
    let emoji = '😻';
    println!("c = {}, z = {}, emoji = {}", c, z, emoji);

    // multiple
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;

    // array
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // [3, 3, 3, 3, 3]


}