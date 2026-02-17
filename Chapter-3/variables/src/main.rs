fn main() {
    // ----------- IMMUTABILITY -----------
    //  let x = 5;    gives immutability error

    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // --------- SHADOWING ----------
    /* Shadowing is different than mutability because the compiler will throw an error without ~let~
     * Another difference is that, since we're effectively creating a new variable, we can change
     * the data type as well. An example at the end.
     */
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y inside the scope is: {y}");
    }

    println!("The value of y is: {y}");

    /* Here, the value of x is first borrowed when we use let x = , then again we repeat
     * this within the inner scope, which changes the value of x to 22, thus shadowing the
     * second let x. However, once the scope ends, the overshadowing ends and the shadowing
     * of the original value of 5 returns to the second let, storing the value of 6.
     */

    // An example of changing data type by shadowing. Prevents from having to make stupid new names
    let spaces = "   "; //If we use mut, we'll get a compile time error.
    let spaces = spaces.len();
}
