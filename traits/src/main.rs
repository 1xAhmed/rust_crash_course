fn main() {
    
    // Traits are similar to interfaces in other languages
    // ust takes the commposition over inheiritance approach

    struct RedFox {
        enemy: bool,
        life: u32,
    }


    // Traits define required behavior, in other words, functions and methods that a struct must implement if it wants to have that trait.
    // Noisy trait specifies that the struct must have A method named get noise that returns a borrowed string slice
    trait Noisy {
        fn get_noise(&self) -> &str;
    }

    // let's add an implementation of the noisy trait for red fox.
    // We implement the noisy trait for the redfox struct and our implementation of the required get_noise method is meow
    impl Noisy for RedFox {
        fn get_noise(&self) -> &str { "Meow?" }
    }
    

    // Once we have a trait involved, we can start writing generic functions that accept any value that implements the trait. This function takes an item of type T, which is defined to be anything that implements the noisy trait. The function can use any behavior on item that the noisy trait defines. So, now we have a generic function that takes any type as long as it satisfies the noisy trait.
    fn print_noises<T: Noisy>(item: T) {
        println!("{}", item.get_noise());
    }

    // As long as one of the either the trait of struct is defined in the project. You can implement any trait for any struct.
    
    // That means you can Implement your traits on any types from anywhere including built-ins or types you import from some other package and on your struts. You can Implement any trait whether built-in or from some project.

    impl Noisy for u8 {
        fn get_noise(&self) -> &str {"BYTE!"}
    }

    print_noises(5_u8); // prints "BYTE!"


    // Theres a special trait called copy, if your type implements copy, then it will be copied, instead of moved and move situations. This makes sense for small values that fit entirely on the stack. Which is why the simple primitive types like integars, floats and booleans implement copy. If a type uses the Heap at all, then it cannot implement copy.You can opt in to implementing copy with your own type, if your type only uses other copy types.

    // Note that the traits implement inheritance. So a trait can inherit from another trait. That makes it possible to have a trait inheritance hierarchy. 

    // If you want to implement default trait behavior inside your trait definition, instead of ending your function or method with semi-colon, add a block with your default behavior

    trait Run {
        fn run(&self) {
            println!("I'm running"); 
        }
    }

    // Then when you implement the trait for your struct, just don't provide a new definition for the method whose default implementation you want to use,the presence of an implemenation will override the default.
    // The robot struct in the middle implements the run trait but it doesn't override the default run method. So when robot.run is called from main at the bottom, it executes the default run method defined on the trait

    struct Robot {}
    impl Run for Robot {}

    let robot = Robot {};
    robot.run();


    // NOTE: You can't define fields as part of the traits that may be added in the future. If someone can figure out the right way to do it. In the meantime the workaround is to Define Setter and getter methods in your train.

}

