fn main() {
    // enums and rust are more like algebraic data types and Haskell then see like enums.

    // You specify an enum with a keyword enum the name of the enum in capital CamelCase and the names of the variants in a block.
    enum Color {
        Red,
        Green,
        Blue,
    }

    // You can leave it like that, then it is normal enum like C. Just namespace into the enum and away you go.
    let color = Color::Red;

    // However, the real power of a rust enum comes from associating data and methods with the variants

    // You can always have a named variant with no data. A variant can have a single type of data, a tuple of data or an anonymous struct of data.
    // an enum is sort of like a union in C only so much better.
    #[derive(Debug)]
    enum DispenserItem {
        Empty,
        Ammo(u8),
        Things(String, i32),
        Place {x: i32, y: i32},
    }


    // If you create an enum, the value can be any one of these varants. For example, your `DispenserItem` could be an empty with no data associated with it but you can tell that its an empty.
    use DispenserItem::*;
    let mut item = Empty;    
    println!("{:?}", item);
    
    // Or it could be an ammo with single byte in it
    item = Ammo(69);
    println!("{:?}", item);

    // Or it could be a things with a string and assigned 32-bit integer in it
    item = Things("hat".to_string(), 7);
    println!("{:?}", item);

    // or it could be a place with X and Y i32
    item = Place{x: 24, y: 48};
    println!("{:?}", item);

    // It can be only one of those but one at a time.

    // Evem better, You can implement functions and methods for an enum. 
    impl DispenserItem {
        fn display(&self) { }
    }

    // You can also use enums with generic.
    // Option is a generic enum in the standard library, the T and angle bracket means any type, you don't have to use T, you could use some other valid identifier like `MYTYPE> but the idiomatic thing to do in Rust is to use T or some other capital letter. 
    // The `Option` enum represents when something is either absent or present. If you're tryinh to reach for a null or null value, use Option in rust. You either have some value wrapped in the sum variant or you have none
    enum Option<T> {
        Some(T),
        None,
    }

    // Because enums can represent all sorts of data, you need to use patterns to examine them. If you want to check for a single variant, you use the `if let` expression. If let takes a pattern that will match one of the variants. If the pattern does match then the condition is true and the variables inside the pattern are created for the scope of the if let block. If the pattern doesn't match, then the condition is false. This is very handy if you care about one variant matching or not but not as great, if you need to handle all the variants at once.
    if let Some(x) = my_variable {
        println!("value is {}", x)
    }

    // In that case you use the match expression, which is match. A variable whose type supports matching like an enum. The body of the matching braces where you specify patterns followed by double arrows which are equal signs followed by greater than symbols pointing to an expression that represents the return value of that arm of the match. 
    // Match expressions require you to write a branch arm for every possible outcome. In other words, the patterns in a match expression must be exhaustive. 
    match my_variable {
        Some(x) => {
            println!("value is {}", x);
        },
        None => {
            println!("no value");
        }
    }

    // A single underscore all by itself is a pattern that matches anything and can be used for default or anything else branch
    match my_variable {
        _ => {
            println!("who cares");
        }
    }

    // Note that, even though you will often see blocks as expression for branch arm, any expression will do including things like function calls and bear values. Either all branch arms need to return nothing or all branch arms need to return the same type. Remember that if you actually use the return value of an expression that ends in a curly brace like match if let or if or a nested block. Then you have to put a semicolcon after the closing brace. If you dont use the return value of a braced expression, then rust lets you cheat and leave off the semicolon.
    match my_variable {
        Some(x) => x.squared() + 1,
        None => 42,
    }




    // Let's look a little more option. Here's the definition of option again. Option is used whenever something might be absent here is how you could create a non variant of an option. I specified the type that some will wrap an angle brackets after the option notice that I don't have a use statement bringing into scope option or its variants sum or none from the standard Library since option and its variants are used so much. They're already included in the standard Prelude, which is the list of items from the standard library that are always brought into scope by default.
    let mut x: Option<i32> = None

}
