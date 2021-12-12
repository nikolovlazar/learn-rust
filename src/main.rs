/*
 * Rust is a statically typed language, which means that it must know the types of all variables at compile time.
 * That's why every value in Rust is of a certain data type. There are two subsets: scalar and compound.
 */

fn main() {
    /*
     * A scalar type represents a single value. There are four primary scalar types in Rust:
     * - integers
     * - floating-point numbers
     * - booleans
     * - characters
     */

    // ========= INTEGERS =========
    /*
     * An integer is a number without a fractional component. Integers come in six variants:
     * - 8-bit
     * - 16-bit
     * - 32-bit
     * - 64-bit
     * - 128-bit
     * - arch
     *
     * Each variant can be either Signed or Unsigned.
     * Signed and Unsigned refer to whether it's possible for the number to be negative.
     * Signed integers can be negative (they can have a sign, a "minus").
     * Unsigned integers are always positive (we don't write +6, just 6).
     */
    let x: u32 = 5;
    println!("x is an unsigned 32-bit integer with the value of {}", x);

    /*
     * Rust is also capable of inferring the type of the value, but not always.
     * In the case below, the type of y is i32, a default type that Rust assigns for integers.
     */
    let y = 15;
    println!("Type type of y is i32, inferred by the value of {}", y);

    // ========= FLOATS =========
    /*
     * A floating-point number is a number with decimal points. We'll call them floats from here on.
     * Rust have two primitive types for floats: 32-bits and 64-bits (f32 and f64).
     * The 32-bit float is also called a single precision float.
     * The 64-bit float is a double-precision float.
     * The default is the 64-bit float because on modern CPUs it's rougly the same speed as the 32-bit one,
     * but it's capable of more precision.
     */
    let pi: f32 = 3.14;
    println!("The value of pi is: {}", pi);

    // ========= BOOLEANS =========
    /*
     * Just like other programming languages, a Boolean type in Rust has two possible values: true and false.
     * Booleans are one byte in size.
     * We can define booleans using the bool keyword.
     */
    let is_active: bool = true;
    println!("Is the user active: {}", is_active);
}
