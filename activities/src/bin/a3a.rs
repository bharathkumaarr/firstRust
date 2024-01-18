// Topic: Flow control using if..else
//
// Program requirements:
// * Displays a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
// Notes:
// * Use a variable set to either true or false
// * Use an if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn main() {
  // variable set to either true or false
  let my_bool = true;
  // if..else block to determine which message to display
  if my_bool == false {
    println!("it is true");
  } else {
    print!("it is false");
  }
}
