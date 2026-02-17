fn main() {
    another_function(5);
    num_and_char(69420, 'M');

    let x = user_value(67);
    println!("The function returns {x}.");
}

// In Rust, type of parameter must always be declared in function signature
fn another_function(x: u8) {
    println!("The value of x is: {x}.");
}

// Multiple parameters seperated by comma, didn't even wanna write this down, just looked empty
fn num_and_char(num: i32, character: char) {
    println!("The num and char is {num} and {character}.");
}

/* For returning values, the return type must be specified in the function header.
 * The return value is synonymous with the final expression in the function body.
 * You can return early from a function using the return keyword. The line is an
 * expression since it doesn't end with a semicolon. Statements do not return any values
 * while expressions do. In C, assignment is an expression whereas in Rut, it is not.
 */
fn user_value(x: i32) -> i32 {
    x
}
