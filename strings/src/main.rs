fn main() {

// There are atleast SIX types of strings in rust standard library, but we mostly care about TWO of them to overlap each other
        // 1. `str` string slice almost always seen as borrowed string slice `&str`. A literall string is always a borrowed string slice. A borrrowed string slice is often reffered to string which will be confusing because other string type is also called `String`.

        // A borrowed string slice is internally  made up of a pointer to some bytes and a length.
        // A `String` is made up of a pointer to some bytes, length and a capacity that maybe higher then the one currently being used.

        // In other words, a borrowed string slice is a subset of strings in more ways then one. Which is why they share a bunch of other characterisitics. For example, both string types are valid UTF-8, by defination, compiler enforecement and by runtime checks.

        // Also strings cannot be indexed by character position.Indexing operation on standard library collections are aways guranteed to be constant time operations. You cant do that with strings because the bytes, which are indexable aren't guranteed to be what poeple want, when they index them to be strings. WHen presented with the string, you have some options
        // You can use the bytes method `word.bytes();` to access the vector of UTF-8 bytes, which you can index into it if you want, since bytes are fixed size. This wors fine for simple English text, as long you stick to the portion of ASCII. You can use the chars method `word.chars();` to retreive an iterator that you can use to iterate through unicode scalers

        // The biggest difference between the two is that the data in borrowed string type `&str` can't be modified, while the data in `String` can be modified
        let msg = "Hello";

        // 2. `String`, we will often treat the string by calling the `"string".to_string();` on a borrowed string type. Or by passing the string type from string from `String::from("string");

    println!("Hello, world!");
}
