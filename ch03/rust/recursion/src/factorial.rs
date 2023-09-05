/// Calulates the factorial of the given number.
///
/// # Arguments
///
/// * [`number`] [`u64`]: Number to calculate the factorial.
///
/// # Returns
///
/// [`u64`]: The factorial of the given number.
fn factorial(number: u64) -> u64 {
  // Base case
  if number == 1 || number == 0 {
    1
  }
  // Recursive case
  else {
    number * factorial(number - 1)
  }
}

fn main() {
  println!("factorial(0) = {}", factorial(0));
  println!("factorial(1) = {}", factorial(1));
  println!("factorial(3) = {}", factorial(3));
  println!("factorial(5) = {}", factorial(5));
  println!("factorial(8) = {}", factorial(8));
  println!("factorial(15) = {}", factorial(15));
  println!("factorial(20) = {}", factorial(20));
}
