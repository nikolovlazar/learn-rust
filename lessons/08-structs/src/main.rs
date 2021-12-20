/*
 * Structs are similar to tuples. They can be different types, just like the tuples. But, each piece of data
 * in the structs can be named, so it's clear what the values mean. The ability to name every piece of data
 * makes the structs much more flexible than the tuples. We don't need to rely on the order of the data to
 * specify or access the values from the struct.
 *
 * We define structs using the keyword struct, followed by its name and then curly brackets. Inside of the
 * curly brackets we define the values of the struct. Those values are called fields.
 */

struct Lesson {
    name: String,
    duration: u32,
    url: String,
    is_published: bool,
}

fn main() {
    println!("========================");
    /*
     * Then, to create an instance of a struct we defined, we just create a variable and assign it to the
     * name of the struct, followed by curly brackets, and assign values to the fields.
     */

    let learn_rust = Lesson {
        name: String::from("Learn Rust"),
        duration: 24,
        url: String::from("https://github.com/lazarnikolov94/learn-rust"),
        is_published: true,
    };

    /*
     * Unlike tuples, we don't need to destructure the struct and obtain all of the values. Using dot
     * notation we can access an individual field from the struct:
     */

    println!("Lesson name: {}", learn_rust.name);
    println!("Lesson duration: {}h", learn_rust.duration);
    println!("Lesson url: {}", learn_rust.url);
    println!("Lesson published: {}", learn_rust.is_published);

    println!("========================");

    /*
     * If we want to change the fields' values after initialization, we need to make the whole instance
     * mutable. Rust doesn't allow us to mark certain fields as mutable.
     */

    let mut beginners_guide_nextjs = Lesson {
        name: String::from("The Beginner's Guide to Next.js"),
        duration: 2,
        url: String::from("https://egghead.io/courses/the-beginners-guide-to-nextjs"),
        is_published: false,
    };

    beginners_guide_nextjs.is_published = true;

    println!("Lesson name: {}", beginners_guide_nextjs.name);
    println!("Lesson duration: {}h", beginners_guide_nextjs.duration);
    println!("Lesson url: {}", beginners_guide_nextjs.url);
    println!("Lesson published: {}", beginners_guide_nextjs.is_published);

    println!("========================");

    /*
     * We can also use the Field Init Shorthand when we want to create an instance of a struct and the
     * variables and field names match:
     */

    let chakra_ui_course = create_lesson(
        String::from("Chakra UI Course"),
        1,
        String::from(
            "https://egghead.io/courses/build-a-modern-user-interface-with-chakra-ui-fac68106",
        ),
    );

    println!("Lesson name: {}", chakra_ui_course.name);
    println!("Lesson duration: {}h", chakra_ui_course.duration);
    println!("Lesson url: {}", chakra_ui_course.url);
    println!("Lesson published: {}", chakra_ui_course.is_published);

    println!("========================");

    /*
     * We can also create a new instance of a struct that uses most of the values of another instance
     * using the Struct Update Syntax.
     *
     * Note: The base struct must always be the last field!
     */

    let updated_chakra_course = Lesson {
        is_published: true,
        ..chakra_ui_course
    };

    /*
     * Remember the ownership feature in Rust? The ".." operator is like assignment with "=". It "moves"
     * the values from chakra_ui_course to update_chakra_course, so we cannot use the chakra_ui_course
     * anymore. Each of the fields' ownership is moved into the new instance, and the old instance's
     * value gets dropped. If we only used the is_published field from the old instance, we still
     * could've continued to use the chakra_ui_course, because the is_published field is a boolean,
     * and as we remember, boolean values implement the Copy trait, so no ownership will be moved to
     * the new instance.
     */

    println!("Lesson name: {}", updated_chakra_course.name);
    println!("Lesson duration: {}h", updated_chakra_course.duration);
    println!("Lesson url: {}", updated_chakra_course.url);
    println!("Lesson published: {}", updated_chakra_course.is_published);

    println!("========================");

    /*
     * We can also create structs without named fields that look similar to tuples. Those are called
     * tuple structs. Tuple structs are useful when you want to give the whole tuple a name and make
     * the tuple be a different type from other tuples. Achieving that with regular structs is possible,
     * but naming each field would be too verbosed and redundant.
     */

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("The black color: {} {} {}", black.0, black.1, black.2);
    println!("The origin point: {} {} {}", origin.0, origin.1, origin.2);

    /*
     * We can see that the Color and Point tuple structs are the same. Same number of fields, same type
     * of fields. But, their types are different. For example, a function that takes a parameter of type
     * Color cannot take a parameter of type Point, even though they're exactly the same. Other than that,
     * tuple structs behave exactly like ordinary tuples.
     */

    println!("========================");

    /*
     * You can also define structs that don't have any fields. Those are called unit-like structs, because
     * they behave similarly to the () type. Unit-like structs are useful in situations where you want to
     * implement a trait on some type, but don't have data that you want to store in the type itself.
     */

    struct AlwaysEqual;
    let _subject = AlwaysEqual;
}

fn create_lesson(name: String, duration: u32, url: String) -> Lesson {
    /*
     * Returning a struct from a function can also be done as an expression.
     */
    Lesson {
        name,
        duration,
        url,
        is_published: false,
    }
}
