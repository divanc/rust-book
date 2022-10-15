pub fn print() {
    println!("\n\n3.1 Variables and mutability");

    println!("\nVariables are immutable by default");
    let x = 5;
    println!("let x = 5;");
    println!("The value of x is: {} ({})", x, str_type(&x));
    println!("x = 6; // error[E0384]: cannot assign twice to immutable variable `x`");

    println!("\nVariables can be made mutable with the `mut` keyword");
    println!("let mut y = 5;");
    let mut y = 5;
    println!("The value of y is: {} ({})", y, str_type(&y));
    println!("y = 6;");
    y = 6;
    println!("The value of y is: {} ({})", y, str_type(&y));

    println!("\nConstants are always immutable");
    println!("You have to specify the type of the constant");
    const MAX_POINTS: u32 = 100_000;
    println!("const MAX_POINTS: u32 = 100_000;");
    println!(
        "The value of MAX_POINTS is: {} ({})",
        MAX_POINTS,
        str_type(&MAX_POINTS)
    );

    println!("\nShadowing");
    println!("Using same variable name with `let` keyword you can overshadow previous variable with a new one with the same name");
    println!("let z = 5;");
    let z = 5;
    println!("let z = z + 1;\nlet z = z * 2;");
    let z = z + 1;
    let z = z * 2;
    println!("The value of z is: {} ({})", z, str_type(&z));

    println!("\nShadow variables can have different types");
    println!("let spaces = '  ';");
    let spaces = "   ";
    let spaces = spaces.len();
    println!("let spaces = spaces.len();");
    println!("The value of spaces is: {} ({})", spaces, str_type(&spaces));
}

fn str_type<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}
