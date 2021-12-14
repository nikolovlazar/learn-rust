fn main() {
    /*
     * We define variables using the let keyword.
     * By default, all variables in Rust are immutable.
     */
    let x = 5;

    /*
     * The following command will not work, and Rust's compiler will complain.
     * Uncomment it to see the error.
     */
    // x = x + 1;

    /*
     * We can print out values using the println method.
     * Using variables in the output is done by placing {}, and then providing the variables as arguments.
     * We can use as many variables as we want.
     * The ! after println indicates that we're calling a macro, so println is actually a macro, not a function.
     * Macros don't always follow the same rules as functions.
     */
    println!("The value of x is: {}", x);

    /*
     * If we want to make a variable mutable, we need to use the mut keyword.
     */
    let mut y = 5;

    /*
     * The following command will work, because we've already told Rust that the y variable can change its value.
     */
    y = y + 1;

    println!("The value of y is: {}", y);

    /*
     * Constants in Rust are different than variables.
     * Constants are always immutable, and they can never be mutable.
     * We define constants with the const keyword.
     * The naming convention for constants is all capitals with underscores as spaces (a.k.a. SCREAMING_SNAKE_CASE).
     * When defining constants, we always have to define the type.
     */
    const NUMBER_OF_TRIES: u8 = 10;

    println!("Tries left: {}", NUMBER_OF_TRIES);

    /*
     * There's also a way to redefine immutable variables in Rust.
     * It's called Shadowing.
     * Shadowing is useful if we want to keep the name of the variable, but change the type.
     * For example: parsing a string into a number.
     * Instead of making two variables "input_str" and "input", we can shadow the string variable after parsing.
     */
    let input = "5";
    let input: u32 = input.parse().expect("Invalid number!");

    println!("The number from the input is: {}", input);
}
