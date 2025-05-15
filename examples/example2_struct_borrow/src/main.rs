fn main() {
    println!("Example 2: Struct Borrow");
    // Define a struct that borrows a string slice
    // The 'a lifetime parameter indicates how long the reference must be valid
    struct TextProcessor<'a> {
        text: &'a str,
    }

    // When implementing methods, we need to declare the lifetime parameter
    impl<'a> TextProcessor<'a> {
        fn new(text: &'a str) -> TextProcessor<'a> {
            TextProcessor { text }
        }
        
        fn count_words(&self) -> usize {
            self.text.split_whitespace().count()
        }
    }

    // Create a string that we'll borrow
    let message = String::from("Hello Rust lifetimes");

    // Create a TextProcessor that borrows the message
    let processor = TextProcessor::new(&message);

    // Use the processor
    println!("The text contains {} words", processor.count_words());

    // Example showing why lifetimes are important
    let processor;

    {
        let temporary_text = String::from("This text will not live long enough");
        
        // This line would cause a compilation error if uncommented:
        // processor = TextProcessor::new(&temporary_text);
        
        // temporary_text is dropped at the end of this scope
    }

    // Create data that lives long enough
    let permanent_text = String::from("This text lives long enough");
    processor = TextProcessor::new(&permanent_text);
    println!("Words in permanent text: {}", processor.count_words());
}