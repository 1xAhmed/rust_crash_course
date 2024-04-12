fn main() {
    // You will encounter closures when you want to spawn a thread or you want to do some functional-programming with iterators and in some comman other places in standard library

    // A closure is an anonymous function that can borrow or capture some data from the scope it is nested in.
    // The syntax is a parameter list between two pipes. Without type annotations followed by a block this creates an anonymous function called a closure that you can call later.
    // `|x, y| {x + y}`


    // The types of the arguments and the return value are all inferred from how you use the arguments and what you return.
    let add = |x, y| {x + y};
    add (1, 2);     // returns 3

    // You dont have to have any parameters, you can just leave the parameter list empty `|| {x + y}`. Technically you can leave the block as well `|| {}`. 
    
    // Closure will borrow a reference to values in the enclosing scope. For example
    let s = "strawberry".to_string();
    // And then we create a closure that borrows a reference to `s` which works because print line macro actually wants a reference. Then we assign the closure to the variable `f`. Whenever we call `f` it prints a strawberry.
    let f = || {
        println!("{}", s);
    };
    f();    // prints strawberry
    // This is great if your closure never outlives the variable it is referencing, but the compiler won't let us send this over to another thread because another thread might live longer than this thread.
    
    // Closures also support move semantics, so we can force the closure to move any variables it uses into itself and take ownerwhip of them. Now s is owned by the closure and will live until the closure itself goes out of scope and gets dropped.
    let c = move || {
        println!("{}", s);
    };

    c();    // prints strawberry

    // So, we could send this closure over to another thread or return it as the value of a function or do anything you want with it.


    // If you want to do some functional stype programming, closures will be your close friends. Call iter on a vector to get an iterator and a whole bunch of methods that use closures will be available to you.
    // Here is an example using map and closure to multiply each item in a vector by 3 then, filter in a closure to discard any values that aren't greater than 10 and then fold with an initial value and a closure to sum the remaining values togather. 

    let mut v = vec![2, 4, 6];

    v.iter()
        .map(|x| x * 3)
        .filter(|x| *x > 10)
        .fold(0, |acc, x| acc + x);


}
