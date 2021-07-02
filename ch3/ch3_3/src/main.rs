fn main() {
    println!("Hello, world!");
    another_function(5);

    // statements has no return value
    //let x = 6;
    // error if uncomment below
    // let x = (let y = 6);

    // statements used for assignment
    let y = {
        let x = 3;
        x + 1
    };
    println!("y = {}", y);

    let x = five();
    println!("x = {}", x);

    // let x = plus_one(5);
    // println!("x = {}", x);

}

// where to define function doesn't matter
fn another_function(x: i32) {
    println!("another function. x = {}", x);
}

// function return value
fn five() -> i32 {
    5
}

// function had wrong return type
// fn plus_one(x: i32) -> i32 {
//     x + 1;
// }