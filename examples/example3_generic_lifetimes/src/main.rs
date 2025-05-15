fn main() {
    println!("Example 3: Generic Lifetimes");
    // Function with generic lifetime parameter
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // Struct that holds a reference with a lifetime parameter
    struct StringHolder<'a> {
        content: &'a str,
    }

    // Implementation for the struct
    impl<'a> StringHolder<'a> {
        fn get_content(&self) -> &str {
            self.content
        }
    }

    // Example usage
    let string1 = String::from("long string is long");
    let string2 = String::from("short");

    // Using the generic lifetime function
    let result = longest(&string1, &string2);
    println!("The longest string is: {}", result);

    // Using the struct with lifetime parameter
    let holder = StringHolder { content: &string1 };
    println!("The holder contains: {}", holder.get_content());

    // Demonstrating lifetime constraints
    {
        let short_lived_string = String::from("short-lived");
        let holder2 = StringHolder { content: &short_lived_string };
        println!("Inner holder contains: {}", holder2.get_content());
        // holder2 and short_lived_string are dropped at the end of this scope
    }
}