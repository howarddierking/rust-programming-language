fn main() {
    let s = "Hello";    // string literal; immutable
    let mut st = String::from("hello");
    st.push_str(", world!"); // function appends a literal to a String (apparently in place)
    println!("{}", st);

    // to ensure memory safety, rust considers st invalid once it is set to st2. 
    // Trying to access it will produce a compiler error. Rust calls this a "move"
    let st2 = st;
    // println!("{}", st); // will produce a compiler error

    // using clone to create a copy of the [heap] data
    let cloneStr1 = String::from("hello");
    let cloneStr2 = cloneStr1.clone();

    println!("cloneStr1 = {}, cloneStr2 = {}", cloneStr1, cloneStr2);

    let a = i32::count_ones(i32::MAX);
    println!("{}", a);
    
}
