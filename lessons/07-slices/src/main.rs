/*
 * Slices are another data type in Rust that doesn't have ownership. They let you reference a contiguous
 * sequence of elements from a collection.
 */

fn main() {
    println!("========================");
    /*
     * Let's say we needed to make a function that returns the first word from a string. If no space is
     * found, the whole string needs to be returned. So how do we approach this?
     *
     * We haven't talked about working with parts of a string. But, we could return the index of the
     * end of the word. Let's try that:
     */

    let mut sentence = String::from("Lazar likes the Rust language!");
    let index = first_word_index(&sentence);

    println!("The last letter index of the sentence is: {}", index);

    /*
     * But, what if we then cleared the string? The index variable will still hold the value of 5,
     * even though the string is empty. So if we were to access the element at that index after the
     * sentence gets cleared we would run into a runtime error. We need a way to connect the index
     * with the state of the sentence.
     */
    sentence.clear();

    println!("The last letter index is still {}", index);

    println!("========================");

    /*
     * We can achieve that synchronization using string slices! A string slice is a reference to a part
     * of a string. It looks like this:
     */

    let mut sentence = String::from("Lazar likes the Rust language!");

    let word = &sentence[0..5];

    println!("The first word: {}", word);

    println!("========================");

    /*
     * The syntax is similar to taking a reference to the whole string, but with the extra [0..5] part.
     * That part defines the beginning and the end of the string that we want to "slice". The start is
     * the index of the element we want to begin, and the end is 1 + the index where we want to end.
     *
     * The indices can also be omitted. If we're starting from 0, we can write [..5]. If we're looking
     * to get the part starting from the 5th character, we can omit the end [5..]. If we wrote [..] then
     * we'll get the whole string.
     *
     * With this in mind, let's refactor the first_word_index function to use slices instead.
     */

    let word = first_word_slice(&sentence);

    println!("The first word using slices is: {}", word);

    println!("========================");

    /*
     * The output is the same! But, let's try to clear the sentence and print out the word again:
     */

    sentence.clear(); // error: cannot borrow `sentence` as mutable because it is also borrowed as immutable

    // println!("The first word after clear is: {}", word);

    /*
     * We get the error above. Why? Remember the borrowing rules from last lesson? You can't borrow a variable
     * as mutable if you've already borrowed it as immutable. The clear method borrows the string as a mutable
     * reference, but when we created the slice we've already borrowed part of the string as immutable. Since
     * we're printing the word after the clear, its scope is still active, so the borrowing of mutable and
     * immutable references overlap each other, hence the error. Uncomment out the println! to see it happen.
     *
     * So, rewriting the first_word_index using slices not only did made our API better, but it also eliminated
     * a number of errors at compile time.
     *
     * We can also use slices with other types, not just strings. Here's an array slice example:
     */

    let values = [1, 2, 3, 4, 5];
    let slice = &values[1..3];

    println!("The slice is: {:?}", slice); // the :? mark is used to print the contents of the array slice

    println!("========================");
}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
