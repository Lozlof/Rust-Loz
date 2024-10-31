fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    /*  you can declare a new variable with the same name as a previous variable. 
        Rustaceans say that the first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable. 
        In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends. */

    let y = 100;
    println!("y one: {y}");

    let y = y + 1;
    println!("y two: {y}");

    {
        let y = y * 2;
        println!("y three inner scope: {y}");
    }

    println!("The final value of y is: {y}")

    /*  This program first binds x to a value of 5. 
        Then it creates a new variable x by repeating let x =, taking the original value and adding 1 so the value of x is then 6. 
        Then, within an inner scope created with the curly brackets, the third let statement also shadows x and creates a new variable, multiplying the previous value by 2 to give x a value of 12. 
        When that scope is over, the inner shadowing ends and x returns to being 6. */
    /*  Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. 
        By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed. */
}