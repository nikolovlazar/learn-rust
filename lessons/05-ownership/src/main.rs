/*
 * Ownership is Rust's most unique feature. It's what enables Rust to make memory safety guarantees
 * without needing a garbage collector.
 *
 * All programs have to manage their way they use a computer's memory. Some programs have garbage
 * collectors that run constantly looking for unused memory to clean up. Other programs leave the
 * memory management to the developers by making them explicitly allocate and free up memory.
 * Rust uses a system of ownership which is a set of rules that the compiler checks at compile time.
 * None of the ownership features are slowing down our programs while they're running.
 *
 * When working with many programming languages you don't need to think about the stack and the heap.
 * But when working with Rust, whether a value is in the stack or the heap has an effect on how the
 * language behaves, so it makes you make certain decisions.
 *
 * The stack and the heap are two differently structured memories available to your program while it's
 * running. The stack stores values in the order it gets them, and removes them in the opposite order.
 * Think of it as a stack of plates. When you add more plates, you put them on the top of the pile.
 * When you need a plate you remove one from the top, which is the last plate you've put in the pile.
 * This way of functioning is called last in, first out, or LIFO. Adding data to the stack is called
 * pushing, and removing data is called popping.
 *
 * All of the data pushed into the stack must have a known, fixed size. The data that doesn't have a
 * fixed size is stored on the heap instead.
 *
 * The heap memory is less organized. When you put data in the heap, the memory allocator finds an empty
 * spot in the heap that's big enough, marks it as being in use, and it returns a pointer to that memory
 * address. This process is called allocating on the heap, or just allocating.
 *
 * Pushing values onto the stack is not considered allocating, but it's faster because the allocator never
 * has to search for a place to store the new data. It's always at the top of the stack. Also accessing
 * data is slower on the heap than the stack, because you have to follow a pointer to get there.
 *
 * When your code calls a function, the values passed into the function (including pointers to data on
 * the heap), and the function's local variables get pushed onto the stack. When the function is over,
 * those values get popped off the stack.
 *
 * Keeping track of what parts of code use what data on the heap, minimizing duplicate data on the heap,
 * and cleaning up unused data on the heap are the problems that the ownership system addresses. Once you
 * understand ownership, you won't think about the stack and the heap very often, but knowing that
 * managing heap data is why ownership exists can help explain why it works the way it does.
 */

fn main() {
    /*
     * There are three ownership rules that we need to keep in mind:
     *    - Each value in Rust has a variable that's called its owner
     *    - There can only be one owner at a time
     *    - When the owner goes out of scope, the value will be dropped
     *
     * Let's talk about the scope. Follow the scope function to continue reading.
     */

    scope();
}

fn scope() {
    /*
     * A scope is a range within a program for which an item is valid. If we declared a variable in this
     * function, it will be only valid inside of this function. When this function stops, and the program
     * goes back to the main function, that variable will not be valid anymore. That's why we can't use
     * that variable in the main function. If we declare it in the scope function, we can use it inside
     * of the scope function, but not outside of it. The scope function is a scope itself. Anything we
     * declare with curly brackets represents a scope. Functions, loops, conditional expressions, are
     * all scopes. Let's see an example:
     */
    // loop {
    //     let x = 5;
    //     break;
    // }

    // println!("{}", x);

    /*
     * In the example above, we define a loop and inside we define a variable x. If we try to print the
     * variable x outside of the loop we get an error "cannot find value `x` in this scope". The loop
     * itself represent a scope, and since we defined the variable x inside, it can only be used inside.
     * When the loop ends, the variable x becomes invalid.
     */

    /*
     * A variable becoming invalid is one of the times where Rust drops the value (releases the memory
     * allocated for it). But it's not always the end of the function where all variables become invalid.
     * To explain the rules of ownership, we're going to use the String type.
     *
     * Follow the ownership function to continue reading.
     */

    ownership();
}

fn ownership() {
    /*
     * String literals are different than strings. They are hardcoded values and are immutable. There are
     * cases where we want to take a user input and store it in a string. For these situations, Rust has
     * a second string type "String", which stores its data on the heap because the data size is unknown
     * at compile time.
     *
     * When a variable goes out of scope, Rust calls a special function called "drop", which is the code
     * for releasing the memory. But what if we want multiple values to interact with the same data?
     *
     * Let's say we had the following block:
     */

    let x = 5;
    let y = x;

    println!("The value of y is {}, and the value of x is {}", y, x);

    /*
     * Since integers are simple values, both these variables get pushed onto the stack. But what would
     * happen if these variables were strings?
     */

    let m = String::from("Lazar");
    let n = m;

    println!("The value of n is '{}', and the value of m is '{}'", n, m);

    /*
     * In this case we can see an error happens when we try to use the variable x. That's the second rule
     * of ownership: "there can only be one owner at a time".
     *
     * When we store variables in the heap, we also store some data related to them in the stack as well.
     * That data holds the pointer (location in memory in heap), the length of the data, and the capacity,
     * which is how much data the allocator gave us for that variable.
     *
     * In the example where we're assigning x to y, we're setting the variable y to the same pointer,
     * length, and capacity that the variable x has. But since there can only be one owner at a time, Rust
     * performs a "move" instead of a "copy". It "moves" the ownership from x to y, and therefore we're
     * not allowed to use x anymore.
     *
     * If we actually intend to make a "copy" instead of a "move", we can use a common method called "clone".
     * Here's an example:
     */

    let a = String::from("Lazar");
    let b = a.clone();

    println!("The value of b is '{}', and the value of a is '{}'", b, a);

    /*
     * Now we don't see the error in variable b like we saw above in variable x. In this case, we're actually
     * making a copy, not moving the ownership. Both variables have ownerships to different locations in the
     * heap.
     */

    /*
     * We don't need to use clone when dealing with scalar values (integers, booleans etc...). These are
     * simple values that are stored in the stack, so copies of the actual values are quicker to make.
     * That's why our first example in this function works just fine. Ownership rules do not apply there.
     */

    /*
     * The ownership system gets interesting when we work with functions. Follow the ownership_and_functions
     * function to continue reading.
     */

    ownership_and_functions();
}

fn ownership_and_functions() {
    /*
     * Just like an assignment, passing values into a function will either copy or move them. In the following
     * example, we can see the move/copy in action:
     */

    let name = String::from("Lazar");
    takes_ownership(name);

    let x = 5;
    makes_copy(x);

    println!("We can use x: {}, but not the name variable {}.", x, name);

    /*
     * We could give back the ownership by returning the variable back to its caller, but it's a bit tedious.
     * Here's an example:
     */

    let language = String::from("Rust");
    let lang = takes_and_gives_back(language);

    println!("We can use lang: {}, which is the same language", lang);

    /*
     * It's techically a different variable, but with the same value. If we wanted to return another value
     * from the takes_and_gives_back function, we'd have to return a tuple, and then destructure it. This
     * seems like a lot of work, but luckily Rust already has a feature for this concept which is called
     * references. We'll cover the references in the next lesson.
     */
}

fn takes_ownership(name: String) {
    println!("I'm taking ownership of the name variable: {}", name);
}

fn makes_copy(x: i32) {
    println!("I'm making a copy of the integer variable: {}", x);
}

fn takes_and_gives_back(language: String) -> String {
    println!("Favorite programming language: {}", language);
    language
}
