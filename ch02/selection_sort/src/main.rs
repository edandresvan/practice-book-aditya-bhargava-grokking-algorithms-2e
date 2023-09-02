pub mod selection_sort;

use selection_sort::selection_sort;

fn main() {
  let my_list = vec![5, 3, 6, 2, 2, -1, 10];
  //let my_list: Vec<i8> = vec![];

  println!("Original array: {:?}", my_list);
  println!("Sorted array:   {:?}", selection_sort(&my_list));
  println!("Original array: {:?}", my_list);
}
