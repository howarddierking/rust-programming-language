fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false")
    }

    // if is an expression, so you can use it on the right side of a let statement
    let x = if number < 5 {
        "less than 5"
    } else {
        "greater than or equal to 5"
    };

    println!("{} is {}", number, x);
}
