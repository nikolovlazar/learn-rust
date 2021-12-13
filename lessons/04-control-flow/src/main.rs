/*
 * Loops and conditional branching are the basic building blocks of most programming languages.
 */

fn main() {
    println!("========================"); // these separators are here to make it easier for you to distinguish the output

    // ========= IF EXPRESSIONS =========
    /*
     * An if expression allows you to branch you code depending on a condition you define.
     * The following is an example of an if expression.
     * Try changing the value of x and re-run the program to see the different output.
     */
    let x = 5;

    if x < 10 {
        println!("x is less than 10!");
    } else {
        println!("x is greater than or equal to 10!");
    }

    println!("========================");

    /*
     * In the example above we also provided an else expression.
     * We use the else expression to give the program an alternative block of code to execute if the condition is not met.
     * Else expressions are optional. We can write a single if expression without an else expresion and that would be fine too.
     */

    /*
     * The condition must always evaluate to a boolean. We'll get an error otherwise.
     * Uncomment the following block and see what happens if the condition doesn't evaluate to a boolean.
     */
    // let y = 10;
    // if y {
    //     println!("Oops!");
    // }

    /*
     * We can also handle multiple conditions using the else if expression.
     * Here's an example with four possible outcomes:
     */
    let z = 6;

    if z % 4 == 0 {
        println!("z is divisible by 4.");
    } else if z % 3 == 0 {
        println!("z is divisible by 3.");
    } else if z % 2 == 0 {
        println!("z is divisible by 2.");
    } else {
        println!("z is not divisible by 4, 3, or 2.");
    }

    println!("========================");

    /*
     * Note that in the case of 6, we can only see the "z is divisble by 3.", but not "z is divisible by 2." even though
     * the condition is met.
     * Rust only executes the first condition block that evaluates to true. Once it finds one, it doesn't even check the rest.
     */

    /*
     * We can also use the if in combination with let, since if is an expression.
     * The value will be bound to the expression that evaluates to true.
     */
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The number is: {}", number);

    println!("========================");

    /*
     * In this case though, the return types of all condition blocks must be the same.
     * The following block will not compile. Uncomment it to see the error.
     */
    // let condition = true;
    // let number = if condition { 5 } else { "six" };

    // ========= LOOPS =========
    /*
     * Sometimes we want to execute a code block more than once. For that, we use loops.
     * Rust has three kinds of loops: loop, while and for. Let's check out each of them.
     */

    /*
     * We define loops with the loop keyword. Rust will execute that block until we explicitly tell it to stop.
     * If we don't, Rust will just keep executing it as long as the program is alive.
     * Uncomment the following block to see the loop in action.
     * Do mind that it will never stop, so you have to manually stop it by pressing Ctrl + C on your keyboard.
     * Don't foget to comment it again afterwards.
     */
    // loop {
    //     println!("I'm looping...");
    // }
    //
    // println!("========================");

    /*
     * Rust provides us with a way to break out of a loop from code. We can use the "break" keyboard within the loop
     * when we want to stop executing it.
     * There's also the "continue" keyword, which we can use to skip executing the remaining code of the loop, and move on
     * to the next iteration.
     */

    let mut count = 5;

    loop {
        if count == 0 {
            break;
        }
        println!("{}...", count);
        count -= 1;
    }

    println!("========================");

    /*
     * If we have nested loops, the break and continue keywords will apply to the innermost loop that they're in.
     * In that case, we can use loop labels to specify which loop we want to break out of, or continue in.
     */
    let mut count = 0;

    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break; // will break out of the unlabeled loop (line 116)
            }
            if count == 2 {
                break 'counting_up; // will break out of the labeled counting_up loop (line 112)
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {}", count);

    println!("========================");
}
