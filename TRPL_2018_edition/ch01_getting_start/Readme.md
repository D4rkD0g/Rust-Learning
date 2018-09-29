➜  ch01_getting_start git:(master) ✗ cargo new hello_world
     Created binary (application) `hello_world` project
➜  ch01_getting_start git:(master) ✗ ls
hello_world
➜  ch01_getting_start git:(master) ✗ cd hello_world
➜  hello_world git:(master) ✗ ls
Cargo.toml src
➜  hello_world git:(master) ✗ cat Cargo.toml
[package]
name = "hello_world"
version = "0.1.0"
authors = ["lambda"]
edition = "2018"

[dependencies]
➜  hello_world git:(master) ✗ cd src
➜  src git:(master) ✗ ls
main.rs
➜  src git:(master) ✗ cat main.rs
fn main() {
    println!("Hello, world!");
}

➜  hello_world git:(master) ✗ cargo check
    Checking hello_world v0.1.0 (/Users/lambda/Documents/GitHub/Rust-Learning/TRPL_2018_edition/ch01_getting_start/hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 1.27s
➜  hello_world git:(master) ✗ cargo build
   Compiling hello_world v0.1.0 (/Users/lambda/Documents/GitHub/Rust-Learning/TRPL_2018_edition/ch01_getting_start/hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 2.81s
➜  hello_world git:(master) ✗ ls
Cargo.lock Cargo.toml src        target
➜  hello_world git:(master) ✗ ./target/debug/hello_world
Hello, world!

➜  hello_world git:(master) ✗ cat src/main.rs
fn main() {
    println!("Hello, Lambda!");
    println("Hello, Lambda!");
}
➜  hello_world git:(master) ✗ cargo check
    Checking hello_world v0.1.0 (/Users/lambda/Documents/GitHub/Rust-Learning/TRPL_2018_edition/ch01_getting_start/hello_world)
error[E0423]: expected function, found macro `println`
 --> src/main.rs:3:5
  |
3 |     println("Hello, Lambda!");
  |     ^^^^^^^ did you mean `println!(...)`?

error: aborting due to previous error

For more information about this error, try `rustc --explain E0423`.
error: Could not compile `hello_world`.

To learn more, run the command again with --verbose.

➜  hello_world git:(master) ✗ cargo run
   Compiling hello_world v0.1.0 (/Users/lambda/Documents/GitHub/Rust-Learning/TRPL_2018_edition/ch01_getting_start/hello_world)
error[E0423]: expected function, found macro `println`
 --> src/main.rs:3:5
  |
3 |     println("Hello, Lambda!");
  |     ^^^^^^^ did you mean `println!(...)`?

error: aborting due to previous error

For more information about this error, try `rustc --explain E0423`.
error: Could not compile `hello_world`.

改后

➜  hello_world git:(master) ✗ cargo run
   Compiling hello_world v0.1.0 (/Users/lambda/Documents/GitHub/Rust-Learning/TRPL_2018_edition/ch01_getting_start/hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 1.97s
     Running `target/debug/hello_world`
Hello, Lambda!
Hello, Lambda!
