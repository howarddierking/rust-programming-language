fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1); // a reference prevents ownership from being transferred 

    println!("The length of '{}' is {}", s1, len);

    // let's try mutating something that's borrowed
    // change_string(&s1);
    // just passing the reference here fails because like variables, references are immutable by default
    // need to specify that the reference is mutable
    change_string(&mut s1); // creates a mutable reference

    println!("{}", s1);

    let s2 = String::from("Hello World");

    let first_word = first_word(&s2);
    println!("the first word is {}", first_word);
}

fn calculate_length(s: &String) -> usize {  // specifies that the function expects a reference to a String
    s.len() // interestingly, you can access the reference via dot notation without dereferencing
}

fn change_string(s: &mut String) {  // accepts a mutable reference
    s.push_str(", world!");
}

// fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {    // unlike the above, this enables the function to accept string literals and String instances
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}