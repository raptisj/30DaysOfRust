# Day 1

**Goals:**

-   Get Rust installed on your local machine.
-   Learn what Cargo is.
-   Write you first Hello Rust program.
-   Overview of key concepts.

## Installation

Install Rust with rustup(recommended).

-   Check if you have curl installed.
-   Run in your terminal `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
-   Add `~/.cargo/bin` to your PATH environment variable

To add the ~/.cargo/bin to your PATH environment you open your .bashrc file and paste this at the bottom. This may vary depending on your operating system.
. . .

```
export PATH="$HOME/.cargo/bin:$PATH"
```

If you run rustc --version in your terminal you should get your Rust version.

```
$ rustc --version
rustc 1.45.2 (d3fb005a3 2020-07-31)
```

You also need you have build-essential installed. If you are in a Linux system like me you install it by running:

```
sudo apt-get install build-essential -y
```

Link to video: [Install Rust in Linux](https://www.youtube.com/watch?v=PHBdlGgCrWw)

## Cargo

Cargo is the official package manager of Rust. You can think this like what is Composer for PHP or NPM for JavaScript.

## Hello Rust

To create a new Rust project you run in your terminal

```
$ cargo new testProject --bin
```

If you ls in your project you will see a src folder which contains a main.rs file, and a Cargo.toml file. In your Cargo.toml file you have some info about your project (name, author, version . . .) and dependencies among other info. Looks something like this:

```
[package]
name = "myproject"
version = "0.1.0"
authors = ["John Raptis <johnsitpars@gmail.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

In our main.rs file we have our main function that prints a string.

```rust
fn main() {
  println!("Hello Rust!!");
}
```

To run this program we head to our terminal in the src directory.

First we create an executable . . .

```
rustc main.rs
```

. . . and then run it

```
./main
```

which outputs

```
Hello Rust!!
```

or using Cargo we can run cargo run

```
$ cargo run
   Compiling myproject v0.1.0 (/home/john/Documents/myproject)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/myproject`
Hello Rust!!
```
