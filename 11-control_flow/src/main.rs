fn main() {

    // If is an expression, not statement. Statements dont return values, expressions do.



    let num = 4;
    let msg;
    if num == 5 {
        msg = "five";
        println!("{}", msg);
    } else if num == 4 {
        msg = "four";
        println!("{}", msg);
    } else {
        "other";
    }


    // Meaning we can change this code into this
    // 4 things to note
        // 1. There are no semi-colons after the branch values. To make it so that the values can return from the block as tail expressions
        // 2. We can't use `return` for this purpose, even we wanted to because return only applies to blocks that are function bodies.
        // 3. All the blocks have to return the same type
        // 4.There is a semi-colon at the end of the `if` expression. If you don't use the  value of an if expression, then rust will let you cheat and leave off the semi colon, but if you do use the value of if expression in a statement, then you need to put a semi colon after it, before starting any other statement in a block
    let num = if num == 5 {
        "five"
    } else if num == 4 {
        "four"
    } else {
        "other"
    };

    println!("{}", num);


    loop {
        break;
    }

    'bob: loop {
        loop {
            loop {
                break 'bob;
            }
        }
    }

    // 'alice: loop {
    //     loop {
    //         loop {
    //             continue 'alice;
    //         }
    //     }
    // }


    // while have all the behavious of unconditional loop, also they terminate the loob when their condition evaluates to false, that is the exact boolean value of false, remember rust refuses to coearce expressions to booleans
    // while dizzy() {
    //     // do stuff
    // }

    for num1 in [24,5,3,53,0,53,4,0].iter() {
        println!("{}", num1);
    }


    // For loop can take a pattern to destructure the item it receives, and bind the insight parts to the variables just like the `let` statement, just in this case, the variables are local to the body of the for loop
    let array = [(12,5), (13,5)];

    for (x,y) in array.iter() {
        println!("{} {}", x,y);
    }


    // Ranges with for loop, the syntax for range is two dots `..`, seperating the start and end points. The start is inclusive and end is exclusive, so this will count to 9, and if you use `..=` then end will be inclusive as well.
    for num in 0..10 {
        println!("{:?}", num);
    }
}
