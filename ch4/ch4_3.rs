fn main() {
    let s = String::from("hello world");
    // don't do this
    // let mut s = String::from("hello world");
    let f = first_word(&s[..]);
    println!("{}", f);
    let another_s = "How are you?";
    let another_f = first_word(another_s);
    println!("{}", another_f);
    
}


// Get the first word of a string
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}