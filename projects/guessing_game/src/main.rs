use std::io; // To obtain user input and then print the result as output, we need to bring the io input/output library into scope.
use rand::Rng; // First we add the line use rand::Rng;. The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.
use std::cmp::Ordering; /* The Ordering type is another enum and has the variants Less, Greater, and Equal. 
                           These are the three outcomes that are possible when you compare two values. */

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
  
   /* We create a variable named guess. But wait, doesn’t the program already have a variable named guess? It does, but helpfully Rust allows us to shadow the previous value of guess with a new one. 
      Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess, for example.
      Know that this feature is often used when you want to convert a value from one type to another type. */
   let guess: u32 = guess.trim().parse().expect("Please type a number!");

   // This line prints the string that now contains the user’s input. The {} set of curly brackets is a placeholder.
   println!("You guessed: {}", guess);

   /* The cmp method compares two values and can be called on anything that can be compared. 
      It takes a reference to whatever you want to compare with: here it’s comparing guess to secret_number. 
      Then it returns a variant of the Ordering enum we brought into scope with the use statement. 
      We use a match expression to decide what to do next based on which variant of Ordering was returned from the call to cmp with the values in guess and secret_number. */
   match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => println!("You win!"),
   }
   /* A match expression is made up of arms. 
      An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern. 
      Rust takes the value given to match and looks through each arm’s pattern in turn. 
      Patterns and the match construct are powerful Rust features: they let you express a variety of situations your code might encounter and they make sure you handle them all. */
   /* vscodeusr@lozcloud:~/Rust-Loz/projects/guessing_game/src$ cargo run
      Compiling guessing_game v0.1.0 (/home/vscodeusr/Rust-Loz/projects/guessing_game)
      error[E0308]: mismatched types
      --> src/main.rs:53:20
      |
      53    |    match guess.cmp(&secret_number) {
      |                --- ^^^^^^^^^^^^^^ expected `&String`, found `&{integer}`
      |                |
      |                arguments to this method are incorrect
      |
      = note: expected reference `&String`
              found reference `&{integer}`
      note: method defined here
      --> /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/core/src/cmp.rs:838:8

      For more information about this error, try `rustc --explain E0308`.
      error: could not compile `guessing_game` (bin "guessing_game") due to 1 previous error */
   /* The core of the error states that there are mismatched types. 
      Rust has a strong, static type system. However, it also has type inference. When we wrote let mut guess = String::new(), Rust was able to infer that guess should be a String and didn’t make us write the type. 
      The secret_number, on the other hand, is a number type. 
      A few of Rust’s number types can have a value between 1 and 100: i32, a 32-bit number; u32, an unsigned 32-bit number; i64, a 64-bit number; as well as others. Unless otherwise specified, Rust defaults to an i32, which is the type of secret_number unless you add type information elsewhere that would cause Rust to infer a different numerical type. 
      The reason for the error is that Rust cannot compare a string and a number type. */
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