fn main() {
    /* Integer types: u/i8, u/i16, u/i32, u/i64, u/i128, u/isize
     * suffixes: _ (decimal), x (hex), o (octal), b (binary)
     * Floating points: f32, f64 (default)
     * Boolean: true, false. One byte in size.
     * Character: Encased in single quotation unlike string. 4 bytes in size.
     */

    const DECIMAL_EXAMPLE: u8 = 0_30;
    const HEXADECIMAL_EXAMPLE: u8 = 0xFF;
    const OCTAL_EXAMPLE: u8 = 0o36;
    const BINARY_EXAMPLE: u8 = 0b11110;

    println!("All examples are: {DECIMAL_EXAMPLE}, {HEXADECIMAL_EXAMPLE}, {OCTAL_EXAMPLE}, {BINARY_EXAMPLE}.");

    let _x = 6.70; // f64
    let _y: f32 = 6.90;

    // ------------- COMPOUND TYPES -----------------

    // ------ Tuples ------
    // Tuples are fixed in length, each position can have a type.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // To retrieve data, pattern matching can be used. This is called destructuring.
    let (a, b, c) = tup;
    println!("The values in the tuple are: {a}, {b}, {c}.");

    // Direct access can be done by the period(.) operator.
    let second: f64 = tup.1;
    println!("The second element is {second}");

    // Example of empty tuple, called a unit
    let _unit = (); // let unit: () = ();

    // ------- Arrays -------
    /* All elements of an array must have the same type. Arrays in Rust have fixed length.
     * Data in arrays is allocated on the stack.
     */
    let _arr = [1, 2, 3, 4, 5];
    let _arr: [u32; 5] = [1, 2, 3, 4, 5];
    let _arr: [u32; 5];
    let _arr = [3, 5];

    //Direct accessing works with square brackets ([]). Invalid memory can't be accessed since the
    //program will panic.
    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
    println!("The first and second elements in the array are: {first}, {second}.");
}
