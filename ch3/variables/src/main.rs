// constants
// - can be declared at any scope
// - must have type annotation
// - must be set to a non-runtime-computed value
const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5; // `mut` changes the default behavior of variables from immutable to mutable
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    println!("The value of the const MAX_POINTS is {}", MAX_POINTS);

    // shadowing
    let s = 1;
    let s = s + 3;
    let s = s * 2;
    println!("The value of s is {}", s);

    // data types 
    // integers
    let guess: u32 = "42".parse().expect("Not a number.");
    println!("The value of guess is {}", guess);

    // floats
    let f1 = 2.0; // default to f64
    let f2: f32 = 3.0; // use the type annotation to be specific about 32 bit floating point
    println!("A few floating point values are {} and {}", f1, f2);

    // primitive compound types: tuples and arrays
    // tuples
    let tup = (500, 6.4, "howard");
    // in order to get at the individual values of a tuple, you can destructure it
    //this is also apparently part of rusts's pattern matching capability
    let (x, y, z) = tup;    
    println!("middle value of sample tuple is {}", y);
    // you can also use dot notation with an index to access a member
    println!("first value of the sample tuple is {}", tup.0);

    // arrays
    // you can initialize with a type-inferred literal
    let arr1 = [1,2,3,4,5];
    println!("arr1 {:?}", arr1);
    // you can initialize with type annotations
    let arr2: [f32; 3] = [2.2, 5.2, 3.1];
    println!("arr2 {:?}", arr2);
    // you can initialize with a default value (2) for all members
    let arr3 = [2; 3];
    println!("arr3 {:?}", arr3);

}
