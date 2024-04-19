fn main(){
    let s1 = String::from("Hello");
    let s2 = "World!";

    println!("{}", std::mem::size_of_val(&s1)); // Prints 24 = 3x8
    println!("{}", s1.len());      // Prints 5, the size of the String
    println!("{}", s1.capacity()); // Prints 5, the capacity of the String
    println!("{:p}", &s1); // 0x7fff58ffb178, the address in the stack
    println!("{:p}", s1.as_ptr()); // 0x5f373059dba0, the address in the heap

    println!("Immutable: {}", s2);
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
