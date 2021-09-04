fn main() {
    // loop expression
    // runs forever
    let mut i = 0;
    let counter_times_2 = loop {
        if i >= 5 {
            break i * 2;  // break is an expression, so it can return a value
        }
        println!("again!");
        i = i + 1;
    };

    println!("the counter * 2 = {}", counter_times_2);

    // while loop
    let mut j = 3;
    while j > 0 {
        println!("j = {}", j);
        j = j - 1;
    }

    // for loop 
    let numbers = [1,2,3,4,5];
    for number in numbers.iter(){
        println!("the number in the array is {}", number);
    }

    // countdown using for loop, range, and rev[erse]
    for number in (1..4).rev(){
        println!("number  = {}", number);
    }
}
