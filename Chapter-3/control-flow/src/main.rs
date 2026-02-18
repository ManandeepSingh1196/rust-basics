fn main() {
    // -------------------- CONDITIONAL STATEMENTS --------------------
    let num: i8 = 1;

    /* Rust only executes the first true condition, and once it finds one, it doesn't
     * even check the rest.
     */
    if num < 5 {
        println!("The number is less than 5!");
        let num = 5;
    } else if num == 5 {
        println!("The number is equal to 5!");
        let num = 6;
    } else {
        println!("The number is greater than 5!");
    }

    /* Using if in a let statement. Since if is an expression, rather than a statement,
     * it can return value to the let statement. Since any block of code in the expression can
     * execute both the results must have the potential to be of the same type.
     */
    let condition: bool = true;
    let number: u8 = if condition { 5 } else { 6 }; // else {'six'} would return an error.
    println!("The value of number in asssignment by if expression is: {number}.");
}
