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
                break; // will break out of the unlabeled loop (line 128)
            }
            if count == 2 {
                break 'counting_up; // will break out of the labeled counting_up loop (line 124)
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {}", count);

    println!("========================");

    /*
     * Loops can also return values. To do that, we can assign the loop to a new variable, and return the result after the break.
     */
    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };

    println!("The result of the loop is: {}", result);

    println!("========================");

    /*
     * Another type of loops that we mentioned was the while loop.
     * While loops are just like the ordinary loops, with the difference of having a condition.
     * So while that condition is true, the while loop's code will repeat.
     * While loops can be easily achieved using the ordinary loop, if, else and break.
     * But since this pattern is pretty common, Rust has a built-in language construct for it.
     * It's much more cleaner than constructing it with loop, if, else and break.
     * Note that while loops cannot return a value like the ordinary loops.
     */
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!");

    println!("========================");

    /*
     * The third type of loops that we mentioned was the for loop.
     * The for loop is useful when we want to iterate through every element in an array.
     * Technically, you can achieve that with a while loop too. Here's an example:
     *    let a = [10, 20, 30, 40, 50];
     *    let mut index = 0;
     *
     *    while index < 5 {
     *        println!("the value is: {}", a[index]);
     *        index += 1;
     *    }
     *
     * The problem with this is the dependency of the index.
     * If you were to change the array to only have 4 items and forgot to update the while condition,
     * your program will panic. It will throw an "index out of bounds" error.
     * It's also slower, because the compiler adds runtime code to check if the condition is still valid.
     *
     * Using a for loop in cases like these is the more clever, and more safer way.
     * Let's rewrite the loop above using a for loop:
     */
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    /*
     * As you can see, the code is much cleaner than the while loop.
     * Even if we changed the array's size, the loop will still work without having to make any changes to it too. Try it out!
     */

    println!("========================");

    /*
     * The safety and conciseness the for loops makes them one of the most used loops in Rust.
     * We can use it even in situations where we don't neccesarily want to loop through an array, but to simply execute a block
     * a certain number of times.
     * We can achieve that by using a Range, which is a type provided by the standard library that generates all numbers in sequence
     * starting from one number and ending before another number.
     * If we were to rewrite our countdown loop using for and Range, it'll look something like this:
     */

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!");

    /*
     * Don't mind the .rev() method. That's an array method that reverses the values in an array, so [1, 2, 3] becomes [3, 2, 1].
     * This code looks much cleaner than the while loop counter!
     */
}
