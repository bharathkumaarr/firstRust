// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn main() {
// variable set to any integer value
let my_variable = 3;

// if..else if..else block to determine which message to display
if my_variable == 1 {
  println!("it is one");
}else if my_variable == 2 {
  println!("it is two");
}else if  my_variable == 3 {
  println!("it is three");
}else {
  println!("none!");
}


}
