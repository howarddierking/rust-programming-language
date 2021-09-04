fn main() {
    println!("Hello, world!");

    another_function(5);

    println!("2+3 = {}", simple_add(2, 3));
}

fn another_function(x:i32){
    println!("the value of x is {}", x);
}

fn simple_add(x: i32, y:i32) -> i32 {
    // the last expression in a function body is the return value unless using the return keyword earlier in the function
    // NOTE that expressions do not have a semicolon at the end
    x+y
}
