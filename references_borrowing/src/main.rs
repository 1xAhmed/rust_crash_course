fn main() {
    let mut s1 = String::from("abc");


    // Instead of moving the variable, lets use a reference.
    // Here is our do_stuff function, this time it takes a reference to a string.

    // When we call do_stuff, we pass it a reference to `s1`. And `s1` retains the ownership of the value. Do stuff, borrows the reference to the value. The reference, not the value gets moved on the function. At the end of the function, the reference goes out of the scope and gets dropped. So, our borrowing ends at that point
    do_stuff(&mut s1);

    // After the function, we can call `s1` like normal because the value never moved.

    // Under the hood!
    // When we create a reference to the `s1`. Rust creates a pointer to the s1, but you almost never talk about pointers in rust, because Rust automatically handles their creation and destruction for the most part and make sure that they are always valid using the concept called Lifetime.
    // Lifetime can be summed up as a rule that the references are always valid, which means the compiler won't let you create a reference that outlives the data who it ultimatly referencing, and you can never point to null.

    // References default to immutable, even if the value being referenced is mutable, but if we make a mutable reference to a mutable value, then we can use the reference to change the value as well. The syntax is 
        // `&mut s1`
        // &mut String  

    // why didn't we have to de-reference the mutable reference in order to alter s in the do stuff method?
    // We are using the same dot syntax to access a string method on a mutable reference as we do for the value itself.

        
    // So at least when you're dealing with the dot operator,

    // you don't have to worry about whether something is a value or a reference or even a reference of a reference.
    // if we manually dereferenced `s1` it would look like this. `(*s)`

    // `(*s1)` You use an asterisk immediately before a reference to de-reference to the value similar to C.

    // The de-reference operator has pretty low precedence. So you'll sometimes need to use parentheses. With most other operators like the assignment operator, for example, you will need to manually de-reference your reference. If you want to read from or write to the actual value here.

    // s.insert_str(0, "Hi, ");
    // *s = String::from("Replacement");

    // Lets go over what references look like
    // If you have a variable `x` then this
    // `&x` this creates a immutable reference to that variables value then this
    // `&mut x` creates a mutable reference to that variables value.

    // Similarly with types
    // If this is the type `i32` of your value then this
    // `&i32` this is the type of your immutable reference and this
    // `&mut i32` is the type of your mutable reference to that value.

    // Going the other way around
    // `x: &mut i32` if your variable is a mutable reference to a value then 
    // `*x // a mutable i32` de-referencing `x` gives you mutable access to the value and if
    // `x: &i32` is an immutable reference to a value, then
    // `*x // an immutable i32` gives you immutable access to the value


    // since references are implemented via pointers

    // rust has a special rule to keep us safe. At any given time you can have either exactly one mutable reference.
    
    // Or any number of immutable references this rule applies across all threads when you consider that references to a variable may exist in different threads.
    
    // It starts to make it pretty obvious why it's not safe to have multiple mutable references to the same variable at the same time without some type of locking but if all the references are immutable then there's no problem so you can have lots of immutable references spread across multiple threats.
    
    // All these rules. I've been talking about are enforced by the compiler and by enforced I mean compiler errors lots of compiler errors



}

// The `&` before the type indicates the reference to a type.
fn do_stuff(s: &mut String) {
    
    s.insert_str(0, "Hi, ");
    println!("{}", s);
    *s = String::from("Replacement");
    println!("{}", s);

}
