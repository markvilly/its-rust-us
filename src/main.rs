fn main() {
    
    // Data types

    // signed integer -> i32, i8, i16, i24, 
    // let mut int_1: i32 = 10; // (2 ^ 32 - 1) / 2

    // unsigned interger. 
    // let uint_2: u32 = 110; // (2 ^ 32 - 1)
    // let boolean: bool = false;
    // let char1: char = 'a';
    // let float_1: f64 = 10.4;
    // let string_slice_1: &str = "Hello";

    // let string_1:  String = String::from("Hello hello");

    // Immutable and Mutables

    // int_1 = 23;

    let a: i32 = 10;
    let b:i32 = 15;
    let c:i32 = add(a,b);

    println!("The sum of {} and {} is {}", a, b, c);


    //Ownership and Referencing

//    let mut variable_1: String = String::from("Dayo");
//    let variable_2: &String = &variable_1;  // &T referencing w/o taking ownership.
//    let variable_3: &&String = &variable_2; // &&T reference of a reference
//    let variable_4: &String = *variable_3;  // *T dereferences

    // variable_2 = &mut String::from("Brello");

    // println!("Variable two is {}", variable_2);
    
    // variable_1 = String::from("Bello");
    
    // println!("Variable one is {}", variable_1);
    
    // println!("Variable three {}", variable_3);

    // let my_name: String = String::from("Ayomide");

    // let name_2: &String = &my_name;
    
    // print_name(name_2);
    // print_name(name_2);
    // print_name(name_2);
    // print_name(&my_name);

    // COMPOUND DATA TYPES - TUPLES, ARRAYS, VECTORS, STRUCTS, ENUMS
    
    // TUPLES
    // let tuple_1:(i32, bool, &str,char, f64)= (23, true, "Hello", 'm', 25.3);
    // let tuple_i32: i32 = tuple_1.0;
    
    // let (tuple_i32  , tuble_bool,tuple_strr,tuple_char, tuple_f64) = tuple_1;

    // println!("{}",tuple_i32);

    // Array

    // let array_1: [u32; 6] = [1,2,3,4,5,6];

    // Vectors

    // let vector_1: Vec<i32> = vec![1,2,3,4,5,6,7];
    
    let mut new_vec: Vec<bool> = Vec::new();
    new_vec.push(true);

    

    println!("{}",new_vec[0]);

}

// fn print_it(name: Vec<bool>)-> Vec<bool>{

//     println!("{}",name);
// }

fn add(a: i32, b:i32)-> i32 {
    let c: i32  = a+b;
    
    c
}


