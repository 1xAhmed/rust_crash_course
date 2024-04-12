fn main() {
    
    // Structs can have data fields, methods, and associated functions
    // Name of the struct in CapitalCamalCase. See below for example, you can end your last field with comma as well.
    struct RedFox {
        enemy: bool,
        life: u8,
    }

    // Instantiating the struct is straightforward though verbose. You need to specify the value for every single field
    
        let fox = RedFox {
            enemy: true,
            life: 70,   
        };

// Typically, you would Implement an Associated function to use as a Constructor to create a struct with default values and then call that 
// Methods and Associated functions are defined in an implementation block that is separate from the struct definition.

// The implementation block starts with impul and then the name of the struct whose functions and methods you are going to implement.

// This is an Associated function because it doesn't have a form of self as its first parameter. And many other languages you would call this a class method.

    impl RedFox {
        fn new() -> Self {
            Self {
                enemy: true,
                life: 70,
            }
        }
    }

    // The scope operator in rust is double colons, and we use it to access parts of namespace like things. We already used in modules as well.
    // Here we are using it to access an associated function of a struct
    let fox1 = RedFox::new();

    let life_left = fox1.life;
    fox1.enemy = false;
    fox1.some_method();
     

    // Methods are also defined in the implementation block. Method always take some form of self is their first argument
    impl RedFox1 {
        // associated function
        fn function() ...
        // methods
        fn move(self) ...
        fn borrow(&self) ...
        fn mut_borrow(&mut self) ...
    }

}
