fn demo_mutation() {
    let mut x: i32 = 5;
    println!("\n#1 The value of x is {x}");
    x = 6;
    println!("#2 The value of x is {x}\n");
}
fn demo_constant() {
    const THREE_AND_HALF_HOURS_IN_SECS: u32 = 60 * (60 * 3 + 30);
    println!("My const is: {THREE_AND_HALF_HOURS_IN_SECS} and can't be mutable\n");
}
fn demo_shadowing() {
    let x = 42;
    println!("#1 The value of x is {x}");

    let x = x + 27;
    println!("#2 The value of x is {x}");
    {
        let x = x * 2;
        println!("#3 The value of x in the inner scope is: {x}");
    }
    println!("#4 The value of x in the outer scope is: {x}\n");
}
fn change_type_with_shadowing() {
    let spaces = "   ";
    println!("#1 Spaces is of type str: |{spaces}|");
    let spaces: usize = spaces.len();
    println!("#2 Spaces is now of type usize: {spaces}\n")
}

fn main() {
    demo_mutation();
    demo_constant();
    demo_shadowing();
    change_type_with_shadowing()
}
