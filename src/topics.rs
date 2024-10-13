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
