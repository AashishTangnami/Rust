// io : input/ouput library. std - standard library.
use std::io;
use rand::Rng;
use std::cmp::Ordering;

// fn - declares the new function

fn main(){
    // println! is a macro
    println!("Guessing the number");

    // rand::thread_rng function that generates random number.
    // gen_range takes expression as an argument that generates the number.
    // start..=end (1 to 100)-- is inclusive on the lower and uppper bounds.

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win!"),
    }
    // instantiating new variable guess 
    // creates the variable mutable.
    // = binds a variable with something.
    // String::new() -> new instance of String.
    // :: associates the new function with std library.

    let mut guess = String::new();

    // recalling the io library.
    // handle userinput from io: std - input()
 
    io::stdin()
        // read_line is a method.
        // &mut guess - is a argument. with variable type &mut.
        // & - indicates argument is a reference.
        // it lets code to access without copying data into memory multiple times.
        .read_line(&mut guess) //returns result value.
        // result value is an enumeration with each possible state as variant. [Ok and Err]
        
        // handles the exceptiong or errors.
        .expect("Failed to read line");

    // 
    println!("You guessed: {guess}");
}