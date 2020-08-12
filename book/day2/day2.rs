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
