fn main() {
    
    // Data types
    let mut int_1: i32 = 10; // (2 ^ 32 - 1) / 2

    // unsigned interger. 
    let uint_2: u32 = 110; // (2 ^ 32 - 1)
    let boolean: bool = false;
    let char1: char = 'a';
    let float_1: f64 = 10.4;
    let string_slice_1: &str = "Hello";

    let string_1:  String = String::from("Hello hello");

    // Immutable and Mutables

    int_1 = 23;

    let a: i32 = 10;
    let b:i32 = 15;
    let c:i32 = add(a,b);

    println!("The sum of {} and {} is {}", a, b, c)
    // Functions  

}

fn add(a: i32, b:i32)-> i32 {
    let c: i32  = a+b;
    
    c
}

fn add_1(a: i32, b: i32) -> i32{

    a + b

}