pub fn win_message() {
    println!("Holaaa!! Right guess.")
}

pub fn data_types() {
    //Data types
    // using prefix _ for unused variables

    // Integer types
    // signed (i) - signs=[-,+] - can include positive and negative values
    let _x: i32 = -342342343; //32 bits - signed
    let _y: i64 = 4357643756435435344; //64 bits - signed

    // unsigned (u) - no signs - only include positive values
    let _a: u32 = 342342343; //32 bits - signed - {i}
    let _b: u64 = 4357643756435435344; //64 bits - signed - {i}

    // i8/u8 - smaller value
    // i128/u128 - larger value

    // isize / usize - bits will be based on architecture of system the program is running on.

    // Floating-point types

    let f = 2.0; //f64- because rust use f64 as default

    // we can also explicitly gave type annotation like this
    let ff: f32 = 6.6; //this is a f32 type

    // f32 is capable of more precision than f64 but f64 on almost all modern cpu's have mostly same speed as f32.

    // Rust also supports basic operators for addition,substraction,multiplication,division,reminder .
    let sum = 5 + 1;
    let diff = 5 - 1;
    let product = 5 * 1;
    let division = 5 / 1;
    let reminder = 5 % 1;

    // Boolean types
    // keyword : bool
    // true/false
    let xi = true; //x will have bool type implicity
                   // we can also give type annotation explicitly like this
    let hi: bool = false;

    // char types
    // keyword : char
    // includes string and emojis
    let emoj = '😍'; //this will have type char as implicitly

    let l: char = 'a'; //4 bits in size - single quote

    // compound types - 2
    // tuple & arrays

    let tup: (i32, f32, u8) = (6000, 6.0, 255); //multiple types,values but single element

    let tuple = (44, 3.9, 2);

    // destructing the tuple
    let (a, b, c) = tuple;

    println!("the value of a is : {}", a);
    println!("the value of b is : {}", b);
    println!("the value of c is : {}", c);

    // accessing element in tuple using a period(.)
    let accesses_el = tuple.0; //like in array index 0,1,2 and so on

    // Arrays

    let arr = [34, 3432, 223, 12, 37];

    //also can initialize an array with same value for each elements and followed by a semicolon then the length of the array you want
    //something like this
    let arr1 = [10; 3]; //[10,10,10]

    // accessing array elements
    let first = arr[0]; //accessing first element & so on ..

    // if we try to access an element at index which is equal or greater than the array length which is not even existed in the array :-
    let value = arr[10]; // arr length  is 5 but we are trying to access 10th element which is not there in the array.

    // rust will exit out immediately with an err message
    // something like this
    //     thread 'main' panicked at src/main.rs:19:19:
    // index out of bounds: the len is 5 but the index is 10
    // note: run with `RUST_BACKTRACE=1` environment variable to    display a backtrace
}

pub fn functions() {
    // functions declaration
    // use fn keyword for defining functions
    fn sample_fn() {
        // --- logic here
    }

    // function calling
    sample_fn();

    // parameters
    fn p_fn(num: i32) {
        // ----we can acces the num inside the function
        println!("{}", num); // this will print the num variable
    }
    //  ---- must declare type of each parameter

    // statements and expressions

    // statements wont return anything
    // expressions return something

    let x = 6;
    // this is a statement ie it doesnt return any value

    // if we try to assign -- let x = (let x = 6);
    // it will throw an error because we are trying to assign an statment which doesnt return anything

    fn main() {
        let y = {
            let x = 3;
            x
        };

        println!("The value of y is: {y}");
    }

    // this wont throw any error because the block
    // {
    //     let x = 3;
    //     x
    // };
    // is returning the x

    // we can return a value with return keyword or just simply puting the variable or value like this

    fn five() -> i32 {
        5
    }

    // if we use 5;(semicolon) it will throw an error
}

pub fn control_flow() {
    // if expressions
    let x = 6;
    if x == 6 {
        println!("Equal");
    } else {
        println!("nothing");
    }

    // expects bool
    // if we use
    // if x {
    //     println!("number was three");
    // }
    // it will throw this error
    //  --mismatched types
    //  --expected `bool`, found integer

    // Rust will not automatically try to convert non-Boolean types to a Boolean

    // we can handle multiple condition using elseif

    if x == 5 {
        //
    } else if x == 4 {
        //
    } else if x == 9 {
        //
    } else {
        //
    }

    // using if in a let statement
    fn main() {
        let condition = true;
        let number = if condition { 5 } else { 6 };

        println!("The value of number is: {number}");
    }

    // if we use   --let number = if condition { 5 } else { "six" };
    // it will throw an error- if else arm mismatched

    // loop

    // Rust has three kinds of loops: loop, while, and for

    loop {
        println!("hi again");
    } // it will run infinitely until we manually stop the program

    // We can use the 'break' keyword to exit the loop entirely.
    // We can use 'continue' to skip the remaining part of the loop's body and jump to the next iteration.

    // returning value from loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // loop labels for differentiate multiple loops
    // If you have loops within loops, break and continue apply to the innermost loop at that point. You can optionally specify a loop label on a loop that you can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote. Here’s an example with two nested loops:

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    // The outer loop has the label 'counting_up, and it will count up from 0 to 2. The inner loop without a label counts down from 10 to 9. The first break that doesn’t specify a label will exit the inner loop only. The break 'counting_up; statement will exit the outer loop. This code prints:

    // $ cargo run
    //    Compiling loops v0.1.0 (file:///projects/loops)
    //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.58s
    //      Running `target/debug/loops`
    // count = 0
    // remaining = 10
    // remaining = 9
    // count = 1
    // remaining = 10
    // remaining = 9
    // count = 2
    // remaining = 10
    // End count = 2

    // while
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    // This construct eliminates a lot of nesting that would be necessary if you used loop, if, else, and break, and it’s clearer. While a condition evaluates to true, the code runs; otherwise, it exits the loop.

    // for
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    // this will setup a countdown 4,3,2,1
    // 1..4 --ranging 1-4
    // .rev() for reversing the order

    
}
