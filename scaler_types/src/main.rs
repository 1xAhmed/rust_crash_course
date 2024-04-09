// usize  ==> usize is the size of the platform's pointer type and can represent any mamory address in the process. And also the type you will usally use to index an array and vector.


// isize  ==> isize is the same as usize except they use i`i` for their prefix. isize has also the same number of the bits as the platform pointer type. The maximum isize value is the upper bound of the object & array size. This ensures that isize can be used to calculate the differences between pointers and be able to address every byte within a value like struct. If you dont annotate integar literal, then it defaults to i32 because its generally the fastest integar even on 64 bit type architectue


fn main() {
    println!("Hello, world!");
}
