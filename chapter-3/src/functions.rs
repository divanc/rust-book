pub fn print() {
    println!("\nFunctions");
    println!("Functions are declared using the `fn` keyword");
    println!("Functions can be used in any place, if they are declared in the same scope");

    println!("\nIn Rust there are statements and expressions");
    let z = {
        let x = 1;
        let y = 2;
        x + y
    };
    println!(
        "Statements are instructions that perform some action and do not return a value.
        let x = 1; // statement
        let y = (let z = 23); // error: expected expression, found statement (`let`)

        Expressions evaluate to a resulting value.
        let z = {{
            let x = 1;
            let y = 2;
            x + y // expression
        }};
        Value of z is: {z}
    "
    );

    println!(
        "\nReturn values
fn five() -> i32 { 5 }
    ",
    )
}
