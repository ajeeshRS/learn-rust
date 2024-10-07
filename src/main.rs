use rand::Rng; //random number generator - external crate
use std::cmp::Ordering; //comparing with ordering
use std::io; // standard input/output library
mod helper; // declare as a module
fn main() {
    println!("Guessing game!");
    // generating a random number within 0 - 100
    let secret_number = rand::thread_rng().gen_range(0..=100);

    // using an infinite loop to run till gets the right guess
    loop {
        // prints in the same line - { print! }
        // prints in new line - { println! }
        println!("The secret number : {}", secret_number);

        println!("Enter you guess");

        // mutable variable using - { mut } - keyword- initialized with an empty string
        let mut guess = String::new();

        // Using the standardinput method on io to read the user input and it respond back with a result which have Ok || Err and if any err happends expect prints the msg inside
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to get the input");

        // converting  string to unsigned 32 bit int
        // { match } expects the u32 int in this case and if Ok- return num / Err - continue the loop from start
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // prints when the the parsing fails
                println!("Please enter a valid number");
                continue;
            }
        };
        // {} used to print the variables inside a string ,separated by commas
        // can also be used like  {guess}
        println!("Your guess: {}", guess);

        // in this case match expects the guess and the secret_number to compare
        // using Ordering from std::cmp to check the equality
        // using & for referencing the variable (borrowing the value temporarily instead of changing the ownership)
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                helper::win_message();
                // breaks out of the loop
                break;
            }
        }

        // You can also acheive the same as above using if - else
        // if guess < secret_number {
        //     println!("too small");
        // } else if guess > secret_number {
        //     println!("too big");
        // } else {
        //     println!("Hey yoo right guess");
        //     break;
        // }
    }
}
