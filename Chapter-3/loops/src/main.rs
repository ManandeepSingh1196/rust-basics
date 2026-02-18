fn main() {
    // ------------------ LOOPING --------------------
    let mut idx: u8 = 0;
    loop {
        println!("again!");
        idx += 1;
        if idx < 5 {
            continue;
        } else {
            break;
        }
        println!("{idx}");
    }

    let mut counter = 0;

    // You can return the value from a loop by adding the value after the break expression.
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // You can apply labels to loops. They can then be used with break or continue
    // Just learned but there is no postfix operator in Rust.
    let mut count = 0;
    'outer_loop: loop {
        count += 1;
        let mut increase = 2;
        println!("The current count is: {count}.");

        'inner_loop: loop {
            if count >= 10 {
                break 'outer_loop;
            }

            if increase > 0 {
                count += 1;
                increase -= 1;
            } else {
                break 'inner_loop;
            }
        }
    }

    // ---------- While loops ----------
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFT-OFF!!!");

    // ---------- For loops ------------

    /* You can use the while construct to loop over elements in a collection, however,
     * it is error-prone. Using for is a safer and more concise alternative.
     */

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value of element is: {element}.");
    }

    // To run a regular loop, you can use a range, provided by the standard library, works like
    // python. Upper limit is not included, while lower is when using range.
    for number in (1..11).rev() {
        println!("{number}!");
    }
    println!("LIFT-OFF!!!");
}
