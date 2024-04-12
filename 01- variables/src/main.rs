fn main() {
    // Here we declare bunnies and initialize it to the integer value of 2
    // initialize variable without type annotation. 
    // Variables are immutable by default in rust, unless explicitly marked mutable
    let bunnies = 2;
    // bunnies = 5;    // Error!

    // Rust is a strongly typed language. So where is the type annotation? Well, whenever rust configure out the appropriate type for you. You can just leave the type annotation out. You don't have to write anything. When you do annotate the type, it looks like this a colon after the variable name and then the type.

    
    // initialize variable with type annotation.
    // Lets make this mutable with keyword `mut`
    let mut sunny: i32 = 4;

    // Now sunny can be changed
    sunny = 5;

    // initalize multiple variables at once, let statement can destructure the data and initialize in corrosponding pattern on right hand side.
    let (apple, banana) = (8, 5);


    let mut x = 5;  // x is mutable
    let x = x;      // x is now immutable, compiler will optimise this away, so actually nothing happens in assembly code

       // Scope (Block Scope) ends after {}
    {
        // X is shadowed with new value 99
        let x = 99;
        println!("{}", x); // Prints "99"

    }
    println!("{}", x) // Prints "5" because x = 99 was dropped immediatly after scope ended

}
