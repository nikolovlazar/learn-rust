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
     * An integer is a number without a fractional component. They come in six variants:
     * - 8-bit (can hold 2⁸ -> 256)
     * - 16-bit (can hold 2¹⁶ -> 65.536)
     * - 32-bit (can hold 2³² -> 4.294.967.296)
     * - 64-bit (can hold 2⁶⁴ -> 18.446.744.073.709.551.616 😅)
     * - 128-bit (can hold 2¹²⁸ -> 340.282.366.920.938.463.463.374.607.431.768.211.460 😵‍💫)
     * - arch
     *
     * Each variant can be either Signed or Unsigned.
     * Each signed variant can store numbers from -(2ⁿ⁻¹) to 2ⁿ⁻¹-1 inclusive, where "n" is the number of bits the variant uses.
     * For example, the signed 8-bit can store numbers from -(2⁸⁻¹) to 2⁸⁻¹-1, or from -128 to 127.
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
     * We can define booleans using the bool keyword, or directly setting a true/false value.
     */
    let is_registered = true;
    let is_active: bool = false;
    println!("Is the user registered: {}", is_registered);
    println!("Is the user active: {}", is_active);

    // ========= TUPLES =========
    /*
     * The tuple in Rust is a general way of grouping together a number of values that can be of different types.
     * They have a fixed length, so once declared they can't grow or shrink in size.
     * Tuples are created by writing comma separated values inside parentheses.
     * Optionally, you can specify the type of each element.
     */
    let tup = (5, 12.46, true);
    let data: (u8, bool, char, f64) = (150, false, 'R', 13.85);

    /*
     * There are two ways to get individual values out of a tuple.
     * We can use pattern matching to destructure it.
     * This creates three separate variables and assigns the respected values from the tuple.
     */
    let (num, float, boolean) = tup;
    println!("The number value of tup is: {}", num);
    println!("The float value of tup is: {}", float);
    println!("The boolean value of tup is: {}", boolean);

    /*
     * We can also access individual values by using a period, followed by the index of the value.
     * Just like in other programming languages, the first index in a tuple is 0.
     */
    let first = data.0;
    let second = data.1;
    let third = data.2;
    let fourth = data.3;
    println!("The first value of data is: {}", first);
    println!("The second value of data is: {}", second);
    println!("The third value of data is: {}", third);
    println!("The fourth value of data is: {}", fourth);

    // ========= ARRAYS =========
    /*
     * Arrays are another way to hold multiple values, but unlike the tuple, all elements must be of the same type.
     * Arrays in Rust also have a fixed length, just like the tuples.
     * They are useful when you want your data allocated on the stack rather than the heap,
     * or when you want to ensure you always have a fixed number of elements.
     */
    let _nums = [1, 2, 3, 4, 5];
    //  ^ Don't mind the underscore before nums. It's there to get rid of the "unused variable" compiler warning.

    /*
     * You can write an array's type by using square brackets, and include the type of each element, a semicolon,
     * and then the number of elements it'll hold.
     */
    let _nums: [char; 4] = ['R', 'u', 's', 't'];

    /*
     * If we don't have all the values at first, we can resrve the length of the array
     * and pre-populate it with some value.
     * The example below will create an array nums with the values [3, 3, 3, 3, 3].
     */
    let nums = [3; 5];

    /*
     * Just like in th other programming languages, we can access the elements through their index.
     */
    println!("The second element in nums is: {}", nums[1]);

    /*
     * If we try to access an element with index larger than the size of the array we'll get an error.
     * In the following case we'll get the error immediately, because Rust knows that we're trying to access
     * the twentieth element of an array that only has 5 elements.
     * Uncomment the following line to see the error.
     */
    // println!("The twentieth element in nums is: {}", nums[19]);

    /*
     * The same error can happen at runtime as well. For example, getting the element position from the user input.
     * In that case, the program will compile, but when we ask for an element with larger position we'll get
     * a runtime error. Rust will "panic", and the program will exit at the point of using the invalid value
     * in the indexing operation.
     * Make sure to handle those cases accordingly.
     */
}
