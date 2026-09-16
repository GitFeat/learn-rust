fn main() {
    convert_temp(100 as f64);
}

fn convert_temp(fahrenheit: f64) {
    const FREEZING_POINT_DIFF: f64 = 32 as f64;
    const RATIO: f64 = 5.0 / 9.0;

    let fah: f64 = fahrenheit;
    let cel: f64 = (fah - FREEZING_POINT_DIFF) * RATIO;
    println!("La conversion est: {fah} °F == {} °C", cel);
}

// reverse range method
fn _looping_through_rev() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("DECOLLAGE !!!");
}
fn _looping_through_classy() {
    // classical way
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}")
    }
}

fn _looping_through_wrong() {
    // this approach is error-prone
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut i = 0;

    while i < 5 {
        println!("The value in the array is {}", arr[i]);

        i += 1;
    }
}
fn _conditionnal_while_loop() {
    let mut number = 5;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("DECOLLAGE !!!");
}

fn _disambiaguation() {
    // [Disambiguating with Loop Labels]
    // If you have loops within loops, break and continue apply to the innermost loop at that point.
    //  You can optionally specify a loop label on a loop that you can then use with break or continue to specify that
    //  those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn _test() {
    let mut counter = 0;

    let result = loop {
        counter = counter + 1;
        println!("{counter}");
        if counter == 10 {
            break counter * 2; // We can add the value you want returned after the break
        }
    };

    println!("The result x 2 is: {result}");
}

fn _infinity_loop() {
    loop {
        println!("again !")
    }
}

fn _var_condition() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("Value of number: {number}")
}

fn _multiple_if() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

// fn _error_func() {
//     // Get the error:
//     // |     if number {
//     // |        ^^^^^^ expected `bool`, found integer
//     let number = 3;

//     if number {
//         println!("number was three");
//     }
// }
