use std::io;
fn main() {
 printing_array_with_index();
}
fn printing_array_with_index(){
    let arr = [10, 20, 30, 40, 50];
    let mut index = String::new();
    println!("Enter an array index  ");
    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Please type a number!");
    let element = arr[index];
    println!("The value of the element at index {} is: {}", index, element);    
}

// fn playing_with_variables(){
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6; // This line will cause a compile-time error
//     println!("The value of x is: {}", x);

//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);
// }

// fn shadowing(){
//     let x = 5;
//     let x = x + 1;
//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {}", x);
//     }
//     println!("The value of x is: {}", x);
// }

// fn data_types(){
//    let number: u32 = 32; // unsigned 32-bit integer
//    let float_number: f64 = 3.14; // 64-bit floating point
//    let is_active: bool = true; // boolean
//    let character: char = 'R'; // character type
//    let tuple: (i32, f64, u8) = (500, 6.4, 1); // tuple
//    let array: [i32; 5] = [1, 2, 3, 4, 5]; // array  
//    let string: &str = "Hello, Rust!"; // string slice
//     println!("Integer: {}", number);
//     println!("Float: {}", float_number);
//     println!("Boolean: {}", is_active);
//     println!("Character: {}", character);
//     println!("Tuple: ({}, {}, {})", tuple.0, tuple.1, tuple.2);
//     println!("Array: [{}, {}, {}, {}, {}]", array[0], array[1], array[2], array[3], array[4]);
//     println!("String: {}", string);
// }