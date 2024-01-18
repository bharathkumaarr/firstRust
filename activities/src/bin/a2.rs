// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

// function to add two numbers together
fn add(a:i32, b:i32) -> i32 {
  a + b
}
// function to display the result
fn display_result(result: i32) {
  println!("{:?}",result);
}

// "{:?}" token in the println macro to display the result
fn main() {
  let result = add(2,3);
  display_result(result);
}
