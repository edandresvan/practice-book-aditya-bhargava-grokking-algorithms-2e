/// Prints a countdown sequence from the given start number up to one.
///
/// # Arguments
///
/// * [`start`] (`i32`): The number with which the countdown will begin.
pub fn countdown(start: i32) {
  // Base case
  if start < 1 {
    println!();
  }
  // Recursive case
  else {
    print!("{}{}", start, if start > 1 { "..." } else { "" });
    countdown(start - 1);
  }
}

fn main() {
  println!("Countdown for 10:");
  countdown(10);

  println!("Countdown for 5:");
  countdown(5);

  println!("Countdown for 3:");
  countdown(3);

  println!("Countdown for 1:");
  countdown(1);

  println!("Countdown for 0:");
  countdown(0);

  println!("Countdown for -3:");
  countdown(-3);
}
