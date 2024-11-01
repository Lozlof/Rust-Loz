// Cargo has generated a “Hello, world!” program for you.
fn main() {
    println!("Hello, world!");
}
/* Cargo expects your source files to live inside the src directory. 
   The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code. */
/* If you started a project that doesn’t use Cargo, as we did with the “Hello, world!” project, you can convert it to a project that does use Cargo. 
   Move the project code into the src directory and create an appropriate Cargo.toml file. */
/* cargo build:
   This command creates an executable file in target/debug/hello_cargo rather than in your current directory. 
   Because the default build is a debug build, Cargo puts the binary in a directory named debug.
   Running cargo build for the first time also causes Cargo to create a new file at the top level: Cargo.lock. This file keeps track of the exact versions of dependencies in your project.
   This project doesn’t have dependencies, so the file is a bit sparse. You won’t ever need to change this file manually; Cargo manages its contents for you. */
/* cargo run:
   We just built a project with cargo build and ran it with ./target/debug/hello_cargo, but we can also use cargo run to compile the code and then run the resultant executable all in one command.
   Using cargo run is more convenient than having to remember to run cargo build and then use the whole path to the binary, so most developers use cargo run.
   Notice that this time we didn’t see output indicating that Cargo was compiling hello_cargo. Cargo figured out that the files hadn’t changed, so it didn’t rebuild but just ran the binary. 
   If you had modified your source code, Cargo would have rebuilt the project before running it. */
/* cargo check:
   Cargo also provides a command called cargo check. This command quickly checks your code to make sure it compiles but doesn’t produce an executable */
/* Building for Release:
   When your project is finally ready for release, you can use cargo build --release to compile it with optimizations.
   This command will create an executable in target/release instead of target/debug.
   The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. */
/* Cargo as convention:
   With simple projects, Cargo doesn’t provide a lot of value over just using rustc, but it will prove its worth as your programs become more intricate. 
   Once programs grow to multiple files or need a dependency, it’s much easier to let Cargo coordinate the build.
   Even though the hello_cargo project is simple, it now uses much of the real tooling you’ll use in the rest of your Rust career.
   In fact, to work on any existing projects, you can use the following commands to check out the code using Git, change to that project’s directory, and build.
    git clone example.org/someproject
$   cd someproject
$   cargo build */