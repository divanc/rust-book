pub fn print() {
    println!("\n\n3.2 Data Types");
    println!("Rust i statically typed language");
    println!("There are 2 subsets: scalar and compound");

    println!("\nStatically ambiguous type would throw a compile error");
    println!("\nlet guess = \"42\".parse().expect(\"Not a number!\"); // error[E0282]: type annotations needed");

    println!("\nlet guess: u32 = \"42\".parse().expect(\"Not a number!\");");
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {guess}");

    println!(
        "\n
Data Types
├── Scalar types
│   ├── Booleans (1 byte)
│   ├── Characters (4 bytes unicode scalar)
│   ├── Integers
│   │   ├── i8 / u8 (8 bits)
│   │   ├── i16 / u16 (16 bits)
│   │   ├── i32 / u32 (32 bits)
│   │   ├── i64 / u64 (64 bits)
│   │   ├── i128 / u128 (128 bits)
│   │   └── isize / usize (arch dependent)
│   ├── Floats
│   │   ├── f32 (32 bits)
│   │   └── f64 (64 bits) (default)
├── Compound types
│   ├── Tuples
│   └── Arrays
\n"
    );

    println!("\nIntegers");
    println!("Signed numbers are stored using two's complement representation");
    println!("Signed variants can store [-1 * 2^(n - 1) ; 2^(n - 1)]");
    println!("Unsigned variants can store [0 ; 2^n - 1]");
    println!("isize / usize are arch dependent. 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture");
    let decimal = 98_222;
    println!("Decimal: let decimal = 98_222; // Value is {decimal}");
    let hex = 0xff;
    println!("HEX: let hex = 0xff; // Value is {hex}");
    let octal = 0o77;
    println!("OCT: let octal = 0o77; // Value is {octal}");
    let binary = 0b1111_0000;
    println!("BIN: let binary = 0b1111_0000; // Value is {binary}");
    let byte = b'A';
    println!("Byte (u8): let byte = b'A'; // Value is {byte}");

    println!("\nTuples");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("let tup: (i32, f64, u8) = (500, 6.4, 1);");
    println!("You can access tuple elements in three ways:");
    let first = tup.0;
    println!("1. via index `tup.0`. Value is {first}");
    let (_x, y, _z) = tup;
    println!("2. via destructurization `let (x, y, z) = tup;`. Value of y: {y}");

    println!("\nArray");
    println!(
        "
    1. Arrays in Rust must have same typed elements
    2. Arrays in Rust have fixed length
    3. Memory for array is allocated on stack"
    );
    let arr = [1, 2, 3, 4, 5];
    println!(
        "let arr = [1, 2, 3, 4, 5]; // or statically typed: let arr: [i32; 5] = [1, 2, 3, 4, 5];"
    );
    println!("let zeros = [0;10]; // 10 elements with value 0");
    let first_element = arr[0];
    println!("let first_element = arr[0]; // Value is {first_element}");
}
