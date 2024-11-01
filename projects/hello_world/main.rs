// The main function always runs first.
fn main() {
    // Rust style is to indent with four spaces, not a tab.
    /* println! calls a Rust macro. 
       If it had called a function instead, it would be entered as println (without the !).
       Using a ! means that you’re calling a macro instead of a normal function and that macros don’t always follow the same rules as functions. */
    // Most lines of Rust code end with a semicolon.
    println!("Hello, world!");    
}