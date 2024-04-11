fn main() {
    
    // Ownership
    // There are three rules to ownership

        // 1. Each value has an owner
            // There is no value in memory, no data, that doesn't have variable that owns it.
        // 2. Only on owner
            // No variables may share ownership, other variables may borrow the value. But only onw variable own's it
        // 3. Value gets dropped if its owner goes out of scope

    // Ownership in Action!

    let s1 = String::from("abc");
    // What happens to the string, is not a copy. At this point, the value of `s1` is moved to `s2`, because only one variable can own the value
    let s2 = s1;
    // println!("{}", s1);     // Error! 

    // Lets walk through it
    // First we create s1, a `pointer = points to the newly allocated bytes on heap`, `len = 3`, & `capacity = 3` get pushed onto the stack. This all togather us the value of string s1.
    // Then, we move s1 value to the `s2`.
    // It works like this, the pointer, length, capacity, all gets copied from s1 and pushed as new values on to the stack as part of the s2. Rust immediately invalidates the s1, its still exists on the stack, the compiler just will now consider s1 uninitialized. And won't let you use s1 after this point.

    // Its more than a shallow copy, its move.

    // Technically, if `s1` were mutable, we could assign some new value to it and then use it again.

    // What if we didn't want to move the value but copy it instead, we would call the clone method instead of the copy. Lets see what it does.
    
    // CLone performs same initial copy of the stack but also copies the heap data and adjusts as `s3` pointers to point to it. In both, move and clone situations, the three rules are satisfied.

    // The stack and heap data, togather make a value. Rust reserves the term copy for only stack data is being copied. If there is heap data and  updates involved, then we use the term clone. In other languages, you might call the `clone` a deep copy
    let s3 = s2.clone();

    // When the value is droppped, that means 
        // 1. the `destructor` is immediately run.
        // 2. The heap portion is immediately freed.
        // 3. The stack portion is immediately popped.
    // Hence, no memory leaks, no dangaling pointers

    // Another move situation.
    let s4 = String::from("xyz");

    // If we pass `s4` into the function, and returns nothing. If we pass `s4` into the function, `s4` is moved into the local variable `s` and do stuff, which means we can't use `s4` anymore, because it got moved.
    do_stuff(s4)
    // println!("{}", s4);     // Error!
    // So what do we do? 1 option is to move it back when we're done. We will just make `s4` mutable, add a return type to `do_stuff`, then return `s4` as tail expression. Which gets moved back out of the function. And used to reinitialize `s4`
    
    
    // This pattern is not ideal as when we pass the value into the function because passing the ownership to function usally means, the function is going to consume the passed in value. For most other cases, you should use references. 
    
}


    fn do_stuff(s: String) {
        // do stuff
    }