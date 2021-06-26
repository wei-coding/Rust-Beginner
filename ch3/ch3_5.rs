fn main() {
    let num = 3;
    if num < 5 {
        println!("True!");
    }else{
        println!("False!");
    }

    // int can't be boolean
    // if num {
    //     println!("3");
    // }
    // use below if needed
    if num != 0 {
        println!("not 0");
    }

    // multiple conditions
    if num % 4 == 0{
        println!("multiply of 4.");
    }else if num % 3 == 0{
        println!("multiply of 3.");
    }else if num % 2 == 0{
        println!("multiply of 2.");
    }else{
        println!("not multiply of 2, 3, 4.");
    }

    // if in statements
    let cond = true;
    let num = if cond {5} else {6};
    println!("num = {}", num);
    // mixed type is not allowed
    // let num = if cond {5} else {"six"};
    // println!("num = {}", num);

    // loop and loop with assignment
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10{
            break counter * 2;
        }
    };
    println!("result = {}", result);

}