/*
 * Functions in Rust are everywhere. You've already seen the most important function in Rust, which is the main function.
 * Functions are declared using the fn keyword, followed by the name of the function, parentheses and then curly brackets.
 * The naming convention for functions in Rust is snake_case.
 */
fn main() {
    println!("Hello from the main function!");

    inner_function();
    print_x(32);
    print_x_and_y(32, 64);
    statements_and_expressions();

    let y = add_three(3);
    println!("The value of y after adding 3 is: {}", y);
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
 * Functions can also accept parameters, which are special variables that are part of the function's signature.
 * We can define the parameters inside of the parentheses.
 * Technically, they're called arguments, but people tend to use arguments and parameters interchangeably.
 * The types of the parameters must always be defined.
 */
fn print_x(x: i32) {
    println!("The value of x is: {}", x);
}

/*
 * When you want a function to accept multiple parameters, just separate them with commas.
 */
fn print_x_and_y(x: i32, y: i32) {
    println!("The values of x and y are {} and {} respectfully.", x, y);
}

/*
 * Rust is an expression-based language. We always need to have that in mind.
 * Function bodies are made up of a series of statements, and optionally end in expressions.
 * Statements are instructions that perform some action and don't return a value.
 * Expressions evaluate to a resulting value.
 * Here are some examples of statements and expressions:
 */
fn statements_and_expressions() {
    // Statement:
    let x = 5;

    // Since statements don't return a value, we cannot do this:
    // let y = (let z = 6); // Uncomment this line to see the error.

    // Expression:
    let x = {
        let y = 3;
        y + 1
    };

    println!("The value of y is: {}", x);

    /*
     * This expression:
     * {
     *  let x = 3;
     *  x + 1
     * }
     *
     * is a block which evaluates to 4. So in the example above, the y variable will have a value of 4.
     * Pay attention to the "x + 1" line without a semicolon at the end, which is not something you've encountered so far.
     * Expressions do not include ending semicolons. If you add a semicolon at the end of an expression,
     * you turn it in a statement, and it won't return a value.
     */
}

/*
 * Functions can also return values. We don't name the return values, we just define their type after an arrow ->.
 * The return value must be of the same type as the final expression in its body.
 * You can return early from a function using the return keyword, but most functions return the last expression implicitly.
 * The following is an example of a function with a return value.
 * Scroll up to the main function to see how we're using the return value.
 */
fn add_three(x: i32) -> i32 {
    /*
     * Just for fun, try adding a semicolon at the end of the expression.
     * It will display an error "expected `i32`, found `()`".
     * The function without an expression implicitly returns () as its body has no tail or a return expression.
     */
    x + 3
}
