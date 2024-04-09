fn main() {
    // initialize variable without type annotation. 
    // Variables are immutable by default in rust, unless explicitly marked mutable
    let bunnies = 2;
    // bunnies = 5;    // Error!

    // initialize variable with type annotation.
    // Lets make thia mutable with keyword `mut`
    let mut sunny: i32 = 4;

    // Now sunny can be changed
    sunny = 5;

    // initalize multiple variables at once, let statement can destructure the data and initialize in corrosponding pattern on right hand side.
    let (apple, banana) = (8, 5);


}
