/*
 * Functions in Rust are everywhere. You've already seen the most important function in Rust, which is the main function.
 * Functions are declared using the fn keyword, followed by the name of the function, parentheses and then curly brackets.
 * The naming convention for functions in Rust is snake_case.
 */

fn main() {
    println!("Hello from the main function!");

    inner_function();
}

/*
 * In this example we've created a new function called inner_function after the main function.
 * Some programming languages won't let you use functions before you define them.
 * Rust doesn't care about where you define your functions. You just need to define them somewhere.
 */

fn inner_function() {
    println!("Hello from the inner function!");
}

/*
 * Functions can also
 */
