use std::io;

// We'll only focus on 'Compouned types' (types composés)
fn demo_tuple() {
    let tup: (i32, f64, u8) = (500, 6.9, 42);

    // Accessing by destructuring
    let (_x, y, _z) = tup; // <== Destructuring ( _ is for unused )

    println!("The value of y is: {y}");

    // Accessing by index
    let first_value = tup.0;
    let second_value = tup.1;
    let third_value = tup.2;

    println!(
        "The values are [{}, {}, {}]\n",
        first_value, second_value, third_value,
    )
}
fn demo_array() {
    let arr = [1, 2, 3, 4, 5]; // Array's length is defined here !

    let a = [10; 5]; // definition of an array with same values [values; length] 

    let first = arr[0];
    let second = a[1];

    println!("Premier elem de arr: {first} \nSecond element de a: {second}\n");
}
fn non_accessible_element() {
    // Construct an array and ask the user what index he wants to display
    // Returns: Value of the input's index OR an error with 'panics'

    // e.g. If the user enter 6 (which is not existing):
    //thread 'main' (7105) panicked at src/main.rs:49:23:
    //index out of bounds: the len is 5 but the index is 6

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
fn main() {
    demo_tuple();
    demo_array();
    non_accessible_element();
}
