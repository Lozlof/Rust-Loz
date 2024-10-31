use std::io; // To obtain user input and then print the result as output, we need to bring the io input/output library into scope.
use rand::Rng; // First we add the line use rand::Rng;. The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.

fn main () {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    /* Next, we’re adding two lines in the middle. 
       In the first line, we call the rand::thread_rng function that gives us the particular random number generator we’re going to use: one that is local to the current thread of execution and is seeded by the operating system. 
       Then we call the gen_range method on the random number generator. 
       This method is defined by the Rng trait that we brought into scope with the use rand::Rng; statement. 
       The gen_range method takes a range expression as an argument and generates a random number in the range. 
       The kind of range expression we’re using here takes the form start..=end and is inclusive on the lower and upper bounds, so we need to specify 1..=100 to request a number between 1 and 100. */

    // Testing purposes.
    println!("The secret number is: {secret_number}");

    println!("input your guess:");

    // Use the let statement to create a variable.
    // Variables are immutable by default, so use mut to specify it is mutable.
    let mut guess = String::new();
    /* The :: syntax in the ::new line indicates that new is an associated function of the String type.
       An associated function is a function that’s implemented on a type, in this case String.
       This new function creates a new, empty string. */
    // In full, the let mut guess = String::new(); line has created a mutable variable that is currently bound to a new, empty instance of a String. 

    // If we hadn’t imported the io library with use std::io; at the beginning of the program, we could still use the function by writing this function call as std::io::stdin.
    /* The line .read_line(&mut guess) calls the read_line method on the standard input handle to get input from the user.
       We’re also passing &mut guess as the argument to read_line to tell it what string to store the user input in. */
    /* The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
       Like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable. */
    io::stdin()
        .read_line(&mut guess) // Notice the lack of ";"
        .expect("Failed to read line");
    // ".expect("Failed to read line");" could have written this code as: io::stdin().read_line(&mut guess).expect("Failed to read line"); 
    /* As mentioned earlier, read_line puts whatever the user enters into the string we pass to it, but it also returns a Result value. 
       Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states. 
       We call each possible state a variant. */ 
    /* Result’s variants are Ok and Err. 
       The Ok variant indicates the operation was successful, and inside Ok is the successfully generated value. 
       The Err variant means the operation failed, and Err contains information about how or why the operation failed. */

    // This line prints the string that now contains the user’s input. The {} set of curly brackets is a placeholder.
    println!("You guessed: {}", guess);
}

// Rust doesn’t yet include random number functionality in its standard library. However, the Rust team does provide a rand crate with said functionality.
/* Remember that a crate is a collection of Rust source code files. 
   The project we’ve been building is a binary crate, which is an executable. 
   The rand crate is a library crate, which contains code that is intended to be used in other programs and can’t be executed on its own. */
// Before we can write code that uses rand, we need to modify the Cargo.toml file to include the rand crate as a dependency.
/* [package]
   name = "guessing_game"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   rand = "0.8.5" */
/* When you do want to update a crate, Cargo provides the command update, which will ignore the Cargo.lock file and figure out all the latest versions that fit your specifications in Cargo.toml. 
   Cargo will then write those versions to the Cargo.lock file. */