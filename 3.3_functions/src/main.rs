fn main() {
    // Entry point
    // println!("Hello, world!");

    // another_function(42);
    // print_labeled_mesurement(60, 's');
    // use_statement();
    let x = arrow_func_nb();
    println!("My function returns: {x}")
}

fn _another_function(x: i32) {
    println!("Another function with value x = {x}");
}

fn _print_labeled_mesurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value.

fn _use_statement() {
    // the block with {} is the expression and the let line the statement
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn arrow_func_nb() -> i32 {
    42 // <== Return value !! (no need to specify with return keyword)*
    // *because it’s an expression whose value we want to return.
}
