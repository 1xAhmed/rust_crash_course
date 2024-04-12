// Always define functions with `pub` keyword to make it visible in the project
// all items in a library are private by default even to binaries in the same project. Let's add Pub in front of the Greet function to make it public now the code works.
pub fn greet() {
    println!("Hi");
}