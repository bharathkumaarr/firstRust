// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {

  // mutable integer variable
  let mut a = 1;

  // loop statement
  loop {

    // Print the variable within the loop statement
    println!("{:?}", a);

    // break to exit the loop
    if a == 4 {
      break
    }
    
    a = a + 1;
    
    
  }
}
