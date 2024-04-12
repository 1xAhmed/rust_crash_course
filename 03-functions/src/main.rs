fn main() {


    // Calling a function looks the same as in most of the languages. There's currently no support for named arguments at the call site. So you need to provide all the values in the correct order. Finally a single rust function does not support variable numbers of arguments or different types for the same argument, but macros such as print line do a macro call looks Just like a function call except the name of a macro always ends with an exclamation mark
    let x = do_stuff(2.0, 12.5);
    println!("{}", x);
}


// You've seen the main function quite a bit already. Let's talk about functions in general functions are defined using the fun. Keyword FN is pronounced fun. The rust style guide says to use snake case for function names lowercase words separated by underscores one. Awesome thing about rust is that functions don't have to appear in the file before code that calls them.

// function parameters are always defined with Name colon type. And of course multiple parameters are separated by a comma you specify the return type after the parameters by adding an arrow pointing to the return type. The arrow is a hyphen and then a greater than symbol and the body of the function is inside a You can return a value from a function using the return keyword as you would expect. There is also a shorthand for returning values. If you leave the semicolon off of the last expression in a block then it will be returned as the value of the block. This is called the tale expression 


fn do_stuff(qty: f64, oz: f64) -> f64 {
    // return qty * oz;   this is okay, but in idiomatic rust, shorter way is preferred

    qty * oz    // leave last expression of the block, then it will return as value of the block. This is called the tail expression
}