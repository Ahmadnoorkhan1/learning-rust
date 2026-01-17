fn main() {
    learning_functions();
}


fn learning_functions(){
    //Functions without parameters
    fn greet() {
        println!("Hello, welcome to learning Rust functions!");
    }
    greet();

    //Functions with parameters
    fn add(a: i32, b: i32) -> i32 {
        return a+b;
    }
    let sum = add(5, 10);
    println!("The sum of 5 and 10 is: {}", sum);

    //Functions with return values
    fn multiply(a: i32, b: i32) -> i32 {
       return a * b;
    }
    let product = multiply(4, 6);
    println!("The product of 4 and 6 is: {}", product);
}