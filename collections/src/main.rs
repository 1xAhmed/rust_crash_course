use std::collections::HashMap;

fn main() {
    // Collections
    // Lets go over some collections, these are all in standard library

    // Vector `Vec<T>` is a generic collection that holds a bunch of one type and is useful where you would use lists or arrays. It is the most commonly used collection by far.
    // When you create a vector, you specify the one type of the object that it will store an angle brackets
    let mut v: Vec<i32> = Vec::new();

    // Once you have the vector, you can push values into it. Vectors act like a stack, so push appends things to the end and pop removes the item at the end of the vector and returns it
    v.push(2);
    v.push(4);
    v.push(6);
    let x = v.pop();    // x is 6

    
    // Since vectors store objects of known size next to each other in memory, you can index into it if your index is out of bounds rust will Panic which is much more helpful than feeding you garbage. So check your length before you index quite often you know, the values of a vector when you create it.
    println!("{}", v[1]);   // prints 4



    // There's a macro called Vex with a lowercase V that makes creating vectors from literal values much more ergonomic. Vectors give you a lot of great low-level control over their behavior and they have a ton of methods to do just about anything. You could possibly want insert remove split splice sort repeat binary search, its all in the standard library.
    let mut v = vec![2,4,6];


    // Hashmap `HashMap<K, V>` is a generic collection, where you specify a type for the key and a type for the value, and you access the values by key. The whole point of a hash map is to be able to loop up and remove values by key in the constant time
    // When you create a hashmap, you specify the type of the key and the type of the value.
    // In this cases, the key is the byte and the value is boolean. 
    let mut h: HashMap<u8, bool> = HashMap::new();

    // Once you have a hashmap, you insert entries with the insert method
    h.insert(5, true);
    h.insert(6, false);

    // Use the remove method to get a value. Remove actually returns an enum called option.
    let have_five = h.remove(&5).unwrap();

    // There are also methods for getting references to values and iterating through keys, values or keys and values, either as immutable or mutable references.


    // There are a bunch of other collections that I will just quickly describe 
    // `VecDeque` uses a ring buffer to implement a double-ended queue, which can efficiently add or remove items from the front and the back but everything else is a little less efficient than a regular vector.

    // `Linkedlist` has the dubious distinction of being quick that adding or removing items at an arbitrary point in the list but slow doing absolutely anything else.

    // `Hashset` is a hashing implementation of a set, that perform set operations really efficiently.

    // `BinaryHeap` is like a priority queue which always pops off the max value.

    // `BTreeMap` & `BTreeSet`are alternate map and set implementations using a modified binary tree.

    // You usually only choose these over the hash variants if you need the map keys or set values to always be sorted
}
