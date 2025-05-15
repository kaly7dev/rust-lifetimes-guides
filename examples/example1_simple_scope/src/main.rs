fn main() {
    println!("Example 1: Simple Scope");
    // Create a variable in the outer scope
    let outer_value = String::from("I'm in the outer scope");
    println!("Outer value: {}", outer_value);

    // Create an inner scope with its own lifetime
    {
        // This variable only lives within this inner scope
        let inner_value = String::from("I only exist in this inner scope");
        println!("Inner value: {}", inner_value);
        
        // We can access outer variables from inner scopes
        println!("Can still access outer value: {}", outer_value);
    } // inner_value's lifetime ends here
    
    // This would fail to compile if uncommented:
    // println!("Trying to access inner value: {}", inner_value);
    
    // Demonstrating reference lifetimes
    let reference_to_outer;
    {
        // This is fine because outer_value lives longer than this scope
        reference_to_outer = &outer_value;
        println!("Reference created in inner scope: {}", reference_to_outer);
    }
    println!("Reference still valid after scope: {}", reference_to_outer);

}