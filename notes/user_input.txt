// To obtain user input and then print the result as output, we need to bring the io input/output library into scope.
use std::io;

fn main () {
    println!("Guess the number!");

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
