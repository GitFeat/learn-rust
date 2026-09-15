fn main() {
    // Entry point
    println!("Hello, world!");

    another_function(42);
    print_labeled_mesurement(60, 's');
    use_statement();
}

fn another_function(x: i32) {
    println!("Another function with value x = {x}");
}

fn print_labeled_mesurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value.

fn use_statement() {
    // the block with {} is the expression and the let line the statement
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
