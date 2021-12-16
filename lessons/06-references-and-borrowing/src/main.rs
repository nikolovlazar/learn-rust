/*
 * In lesson 05 we learned about the Ownership system and what happens when variables go out of scope.
 * We mentioned an idea of a workaround of how to prevent value dropping when arguments go out of scope,
 * but its implementation was a bit tedious and messy.
 *
 * In this lesson we'll learn about the proper way to pass variables as arguments and prevent their
 * ownership from being transfered, and their value being dropped.
 *
 * We can achieve that through referencing.
 */
fn main() {
    /*
     * A reference in Rust allows you to create a "reference" to a value without taking ownership of it.
     * We can create references by prefixing the variable with ampersand (&). The type of the reference
     * will be the type of the referenced variable prefixed with & as well. Let's see an example:
     */
    let name = String::from("Lazar");
    let len = calculate_length(&name);

    println!("The length of '{}' is {}.", name, len);

    /*
     * As you can see, we can continue using the name variable, even though we passed it to the
     * calculate_length function. Correction: we passed its "reference" to the function, not the actual
     * variable.
     *
     * Let's refresh our knowledge about how we stored variables in the heap. In the previous lesson we
     * learned that values with variable size get stored in the heap memory (strings, vectors etc...). When we
     * store them in the heap, we also push an entry onto the stack memory that points to the address of the
     * variable in the heap memory (pointer, length, capacity). When we created a reference to the name variable,
     * we're referring to the pointer that points to the memory address in the heap where the name variable is
     * stored.
     *
     * When we're passing references as arguments in a function, we also need to define them as references in
     * the function declaration as well (fn calculate_length(name: &String) ...). This allows us to use the
     * name variable in the calculate_length scope, but its value doesn't get dropped when the scope ends. We call
     * the action of creating references "borrowing".
     *
     * What happens if we try to modify something we're borrowing? We can't! Just like variables are immutable
     * by default, references are immutable by default as well. Here's an example (that won't work) to prove it:
     * P.S. You won't see the error in your IDE because of the other errors below, but it's there!
     */

    let digits = String::from("abc");

    add_digits(&digits);

    println!("The digits are now: {}", digits);

    /*
     * We can fix that with just a few small tweaks:
     *    - Change "digits" to be mutable: let mut digits = String::from("abc");
     *    - Pass a mutable reference to add_digits: add_digits(&mut digits);
     *    - Update the signature of add_digits to accept a mutable reference: fn add_digits(digits: &mut String)
     *
     * This makes it clear to the Rust compiler that the add_digits is going to mutate the value it borrows.
     * Go ahead and apply these changes. The error should be gone, and you should see "abcdef" in the output.
     */

    /*
     * But, mutable references have one big limitation: you can have only one mutable reference to a certain
     * particular piece of data at a time! We can't do this (make sure you change the digits variable to mutable):
     */
    let ref1 = &mut digits;
    let ref2 = &mut digits; // cannot borrow `digits` as mutable more than once at a time

    println!("{} {}", ref1, ref2);

    /*
     * This error says that our code is invalid becase we're borrowing digits more than once in the same scope.
     * This restriction allows for mutation but in a very controlled way. Most languages allow you to mutate
     * whenever you like, but not Rust.
     *
     * The benefit of having this restriction is that Rust can prevent data races at compile time. Data races
     * are similar as race conditions and happens when these behaviours occur:
     *    - Two or more pointers access the same data at the same time
     *    - At least one of the pointers is being used to mutate the data
     *    - There's no mechanism being used to synchronize access to the data
     *
     * A way to go around this is to put one of the mutable references into its own scope, instead of
     * defining both of them simultaneously.
     */

    {
        // <- new scope!
        let innerRef = &mut digits;
        println!("Inner ref: {}", innerRef);
    } // <- scope ends, innerRef stops existing, so we can safely make another mutable reference after this

    let outerRef = &mut digits;
    println!("Outer ref: {}", outerRef);

    /*
     * A similar restriction happens if we combine mutable and immutable references. So this code won't work:
     */

    let mut person = String::from("Lazar");

    let r1 = &person;
    let r2 = &person;
    let r3 = &mut person; // cannot borrow `person` as mutable because it is also borrowed as immutable

    println!("{} {} {}", r1, r2, r3);
    /*
     * "What?!? How are we supposed to go around this now?". Remember when we talked about scopes and
     * references? A reference's scope starts from where it's defined until the last time is used. Try
     * moving the r3 definition below the println call and add another println for it. Why is the error
     * gone now? Because the scope of r1 and r2 stops at the first println, and they become invalid. After
     * the first println we can safely define r3, because r1 and r2 are not valid anymore, and their scopes
     * are not overlapping (r1 and r2 stops before r3 begins). The ability of the compiler to tell that
     * a reference is no longer being used at a point before the end of the scope is called Non-Lexical
     * Lifetimes (NLL for short).
     *
     * Try adding a new println! below that prints
     * all three references. Even though we moved the r3 below the first println, by printing r1 and r2
     * below we're continuing their scope, so defining r3 will not be allowed again. Now you definitely
     * understand the reference scope!
     */

    /*
     * In languages with pointers there's a concept called Dangling References. It's when a pointer points
     * to a memory address that may have been given to someone else, or freeing some memory while preserving
     * a pointer to that memory. In Rust, the compiler guarantees that references will never be dangling
     * references. If you have a reference to some data, the compiler will ensure that the data will not
     * go out of scope before the reference to the data does.
     *
     * Let's try to create a dangling reference, which Rust will prevent with a compile-time error:
     */

    let reference_to_nothing = dangle(); // the error happens in the signature of the dangle function

    /*
     * The error message contains a key to why this code is a problem:
     * "this function's return type contains a borrowed value, but there is no value for it to be borrowed from"
     *
     * Let's explain why this happens. Follow the dangle_explained function:
     */

    dangle_explained();

    /*
     * Because s in created inside dangle_explained, when the function finishes s will be deallocated. But we
     * tried to return a reference to it. That means that the reference will point to an invalid String! Rust
     * will not allow that. The fix is to return the value itself, not the reference to it. That will work
     * just fine! Ownership is moved out, and nothing is deallocated. Go ahead and try it out!
     */
}

fn calculate_length(name: &String) -> usize {
    name.len()
}

fn add_digits(digits: &String) {
    digits.push_str("def"); // Error: cannot borrow `*digits` as mutable, as it is behind a `&` reference
}

fn dangle() -> &String {
    //         ^ error: expected named lifetime parameter
    let s = String::from("Dangle!");

    &s
}

fn dangle_explained() -> &String {
    //                   ^ we return a reference to a String
    let s = String::from("Dangle!"); // s is a new string

    &s // we return a reference to s
} // here s goes out of scope, and it's dropped. Its memory goes away, but we still have a reference to it!
