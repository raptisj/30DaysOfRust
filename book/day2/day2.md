# Day 2

**Goals:**

- Learn basic syntax
- Build a basic program.

First we want to get user input. In order to do that we need to bring the io library from the Rust standard library known as _std_.

Rust by default brings some types into the scope of our program. The list of things we are likely to use often is imported automatically and it's called the _prelude_.

Everything else must be imported.

To import a library with a use statement.

```
use std::io;

fn main() {
  println!("What is your name?");
}
```

To store the user input we need to create a place for it. Here we use the let statement with it's variable name. The mut keyword means that it's mutable. BY default Rust makes variables immutable. We assign it a new instance of the String type which is provided by the standard library. What this does is it creates an empty string ready to be modified in the future.

```
let mut name = String::new();
```

Note The :: syntax means that new or anything else for that matter is an associated function of the String type.

A bit more on strings. There are two types of strings in Rust. String and \$str. While &str has a borrowed content, String is heap allocated. This has to do with the concept of ownership which we will cover later on. For now just remember that there is a difference between the two.

We made a place for our input. Now we have to get the value of it. Here we will use the stdin function from the io module.

```
io::stdin()
  .read_line(&mut name)
  .expect("Failed to read line");
```

We call the .read_line which takes one argument. It takes the value of what the user typed and place it into a string. The & indicates that the argument is a reference. (More on this later it's a complex topic.).

This basically says make a mutable copy of the name variable and add the input value in to this string.

To handle any possible errors we might get we use the .expect function and pass it a message.

And last to print the result we have a placeholder into a string which our name variable will be displayed.

Placeholder is represented with curly braces.

```
println!("Your name is {}", name);
```

If we run cargo run we get the following:

```
$ cargo run
  Finished dev [unoptimized + debuginfo] target(s) in 0.00s
  Running `target/debug/myproject`
What is your name?
John
Your name is John
```

If we run this it will have a program that asks our name and prints it back to us.

Next we will build a little program that take two numbers, cubes them and returns the sum of the two.

First we ask the user to type a number and store it in a mutable variable. Since the input of the user is has a type of string and we need an integer, we have to convert it. Here we assign the first_number variable and we type annotate it to a primitive type of signed integer i32. We trim it to remove any break lines and make the conversion.

```
use std::io;

fn main() {
    println!("First number to be cubed");

    let mut first_number = String::new();

    io::stdin()
      .read_line(&mut first_number)
      .expect("Failed to read line");

    let first_number: i32 = first_number.trim().parse().expect("Please type a number!");

}
```

Now we create the functions that will handle cubing a number and adding the results.

```
. . .

fn find_cube(x: i32) -> i32 {
   x * x * x
}

fn find_sum(x: i32, y: i32) -> i32 {
  if x > y {
    x + y
  } else {
    y + x
  }
}

. . .
```

Arguments of functions are type annotated as well. The arrow (->) mean that if the function returns a value we have to specify it's type.

After that we pass our first number to the find_cube function and store it's value in a variable. We follow the same process with the second number a well and print the result.

The complete program will look something like this.

```
use std::io;

fn find_cube(x: i32) -> i32 {
   x * x * x
}

fn find_sum(x: i32, y: i32) -> i32 {
  if x > y {
    x + y
  } else {
    y + x
  }
}

fn main() {
    println!("First number to be cubed");

    let mut first_number = String::new();

    io::stdin()
      .read_line(&mut first_number)
      .expect("Failed to read line");

    let first_number: i32 = first_number.trim().parse().expect("Please type a number!");

    let first_cube = find_cube(first_number);

    println!("Second number to be cubed");

    let mut second_number = String::new();

    io::stdin()
      .read_line(&mut second_number)
      .expect("Failed to read line");


    let second_number: i32 = second_number.trim().parse().expect("Please type a number!");

    let second_cube = find_cube(second_number);
    println!("{} cubed is {}. {} cubed is {}. The sum of the two cubes is {}.", first_number, first_cube, second_number, second_cube, find_sum(first_cube,  second_cube));

}
```
