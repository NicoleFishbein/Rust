fn main(){
    let mut s = String::with_capacity(50);

    s.push_str("Hello");
    s.push_str(" World");

    println!("String: {}", s);
    println!("Length: {}", s.len());         // Outputs the current length of the string
    println!("Capacity: {}", s.capacity());

    let s2 = "Immutable";
    println!("BlaBla {}", s2);
}


// fn main() {
//     // Immutable string (string slice)
//     let greeting = "Hello";

//     // Mutable string
//     let mut place = String::from("World");

//     // Print the initial state of both strings
//     println!("{} {}", greeting, place);

//     // Modify the mutable string
//     place.push_str(", Rust!");

//     // Print the final state of both strings after modification
//     println!("{} {}", greeting, place);

//     let mut test = "TEST";
    
//     println!("BlaBla {}", test);

//     let word = String::from("Hello, World!");

//     // RECOMPILE!!!!!

//     // slicing a string
//     let slice = &word[0..5];

//     println!("string = {}", word);
//     println!("slice = {}", slice);
// }
