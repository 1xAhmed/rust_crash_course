fn main() {
    let x = do_stuff(2.0, 12.5);
}


fn do_stuff(qty: f64, oz: f64) -> f64 {
    // return qty * oz;   this is okay, but im idiomatic rust, shorter way is preferred

    qty * oz    // leave last expression of the block, then it will return as value of the block.
}