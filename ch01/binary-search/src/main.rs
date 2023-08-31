use std::fmt::Display;

fn binary_search_8_bits<T>(
  elements: &[T],
  element: &T,
) -> Option<u8>
where
  T: PartialOrd + Display,
{
  // The low and high indexes delimits the subslice or subarray that might
  // contain the required element.
  // Initialize at the lower and upper indexes of the given collection, respectively.
  // Note that both indexes are unsigned integers.
  let mut low_index: u8 = 0;
  let mut high_index: u8 = (elements.len() - 1) as u8;

  // Keep searching until the low index surpases the high index
  // or an internal conditional return happens.
  while low_index <= high_index {
    // Calculate the middle index (half sum) avoiding underflow and overflow problems

    let middle_index: u8 =
      low_index + ((high_index - low_index) as f64 / 2.0).floor() as u8;
    // Suppose that the element at the middle index is the required value
    let guess: &T = &elements[middle_index as usize];

    // If any of the high and low indices reach their extreme value, check whether the
    // element in that position is the desired one.
    // At this point, if the element at either end is not the desired, then the indices
    // should not be modified and instead the function should finish and return None
    if high_index == 0 || low_index == (elements.len() - 1) as u8 {
      return if guess == element { Some(middle_index) } else { None };
    }

    // Check if the assumed element in the middle of the array is the required value.
    if guess == element {
      return Some(middle_index);
    }
    // Check if the required value is on the left half
    else if guess > element {
      // The next search will be at the subarray [low_index, middle_index - 1]. Avoid an overflow error.
      high_index = if middle_index > 0 { middle_index - 1 } else { 0 };
    }
    // Maybe, the required value is on the right half
    else {
      // The next search will be at the subarray [middle_index + 1, high_index]
      low_index = middle_index + 1;
    }
  }
  // Here, the element was not found, so return None
  None
}

/// Gets the index of the specified element in the given collection of elements.
///
/// # Arguments
///
/// * `elements`: Collection of elements.
/// * `element`: Element from which the index is to be searched.
///
/// # Returns
///
/// An `Option` value containing:
///
/// * An `usize` value representing the index if the element was found in the collection.
/// * `None`, Otherwise.
fn binary_search<T>(
  elements: &[T],
  element: &T,
) -> Option<usize>
where
  T: PartialOrd + Display,
{
  // The low and high indexes delimits the subslice or subarray that might
  // contain the required element.
  // Initialize at the lower and upper indexes of the given collection, respectively.
  // Note that both indexes are unsigned integers.
  let mut low_index: usize = 0;
  let mut high_index: usize = elements.len() - 1;

  // Keep searching until the low index surpases the high index
  // or an internal conditional return happens.
  while low_index <= high_index {
    // Calculate the middle index (half sum) avoiding underflow and overflow problems
    let middle_index: usize =
      low_index + ((high_index - low_index) as f64 / 2.0).floor() as usize;
    // Suppose that the element at the middle index is the required value
    let guess: &T = &elements[middle_index];

    // Check if any of the high and low indices reach their extreme values and whether the
    // element in that position is the desired one.
    // At this point, if the element at either end is not the desired, then the indices
    // should not be modified and instead the function should finish and return None.
    if high_index == 0 || low_index == (elements.len() - 1) {
      return if guess == element { Some(middle_index) } else { None };
    }

    // Check if the assumed element in the middle of the array is the required value.
    if guess == element {
      return Some(middle_index);
    }
    // Check if the required value is on the left half
    else if guess > element {
      // The next search will be at the subarray [low_index, middle_index - 1]. Avoid an overflow error.
      high_index = if middle_index > 0 { middle_index - 1 } else { 0 };
    }
    // Maybe, the required value is on the right half
    else {
      // The next search will be at the subarray [middle_index + 1, high_index].
      // (Avoid an overflow error: For debugging: middle index should not be equal to
      //    the size of the elements set)
      if middle_index == elements.len() {
        panic!("The middle index is equal to the the elements size.");
      }
      low_index = middle_index + 1;
    }
  }
  // Here, the element was not found, so return None
  None
}

fn main() {
  let elements = [1, 3, 5, 7, 9];
  println!("Searching index of  3: {:?}", binary_search(&elements, &3));
  println!("Searching index of -1: {:?}", binary_search(&elements, &-1));
  println!("Searching index of  7: {:?}", binary_search(&elements, &7));
  println!("Searching index of 11: {:?}", binary_search(&elements, &11));
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_vector_8_bits() {
    let mut elements: Vec<i16> = vec![
      -128, -127, -126, -125, -124, -123, -122, -121, -120, -119, -118, -117, -116, -115,
      -114, -113, -112, -111, -110, -109, -108, -107, -106, -105, -104, -103, -102, -101,
      -100, -99, -98, -97, -96, -95, -94, -93, -92, -91, -90, -89, -88, -87, -86, -85,
      -84, -83, -82, -81, -80, -79, -78, -77, -76, -75, -74, -73, -72, -71, -70, -69,
      -68, -67, -66, -65, -64, -63, -62, -61, -60, -59, -58, -57, -56, -55, -54, -53,
      -52, -51, -50, -49, -48, -47, -46, -45, -44, -43, -42, -41, -40, -39, -38, -37,
      -36, -35, -34, -33, -32, -31, -30, -29, -28, -27, -26, -25, -24, -23, -22, -21,
      -20, -19, -18, -17, -16, -15, -14, -13, -12, -11, -10, -9, -8, -7, -6, -5, -4, -3,
      -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
      21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
      42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62,
      63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83,
      84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103,
      104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119,
      120, 121, 122, 123, 124, 125, 126, 127,
    ];

    assert_eq!(binary_search_8_bits(&elements, &3), Some(131));
    assert_eq!(
      elements[binary_search_8_bits(&elements, &3).unwrap() as usize],
      3
    );

    assert_eq!(binary_search_8_bits(&elements, &-115), Some(13));
    assert_eq!(
      elements[binary_search_8_bits(&elements, &-115).unwrap() as usize],
      -115
    );

    assert_eq!(binary_search_8_bits(&elements, &-128), Some(0));
    assert_eq!(
      elements[binary_search_8_bits(&elements, &-128).unwrap() as usize],
      -128
    );

    assert_eq!(binary_search_8_bits(&elements, &127), Some(255));
    assert_eq!(
      elements[binary_search_8_bits(&elements, &127).unwrap() as usize],
      127
    );

    assert_eq!(binary_search_8_bits(&elements, &-129), None);
    assert_eq!(binary_search(&elements, &-150), None);

    assert_eq!(binary_search_8_bits(&elements, &128), None);
    assert_eq!(binary_search_8_bits(&elements, &150), None);

    let element_deleted = elements.remove(90);
    assert_eq!(binary_search_8_bits(&elements, &element_deleted), None);
  }

  #[test]
  fn test_vector_16_bits() {
    let mut elements: Vec<i32> = Vec::with_capacity(i16::MAX as usize);

    for number in i16::MIN..=i16::MAX {
      elements.push(number as i32);
    }

    assert_eq!(binary_search(&elements, &-32_768), Some(0));
    assert_eq!(
      elements[binary_search(&elements, &-32_768).unwrap() as usize],
      -32_768
    );

    assert_eq!(binary_search(&elements, &32_767), Some(65_535));
    assert_eq!(
      elements[binary_search(&elements, &32_767).unwrap() as usize],
      32_767
    );

    assert_eq!(binary_search(&elements, &-32_769), None);
    assert_eq!(binary_search(&elements, &-32_780), None);

    assert_eq!(binary_search(&elements, &32_769), None);
    assert_eq!(binary_search(&elements, &32_780), None);

    let element_deleted = elements.remove(45204);
    assert_eq!(binary_search(&elements, &element_deleted), None);
  }

  #[test]
  fn test_vector_small_8_bits() {
    let elements: Vec<i16> = vec![1, 3, 5, 7, 9];

    assert_eq!(binary_search_8_bits(&elements, &3), Some(1));
    assert_eq!(binary_search_8_bits(&elements, &7), Some(3));
    assert_eq!(binary_search_8_bits(&elements, &-1), None);
    assert_eq!(binary_search_8_bits(&elements, &11), None);
  }

  #[test]
  fn test_vector_small() {
    let elements: Vec<i32> = vec![1, 3, 5, 7, 9];

    assert_eq!(binary_search(&elements, &3), Some(1));
    assert_eq!(binary_search(&elements, &7), Some(3));
    assert_eq!(binary_search(&elements, &-1), None);
    assert_eq!(binary_search(&elements, &11), None);
  }

  #[test]
  #[ignore = "i32 can consume memory and time"]
  fn test_vector_32_bits() {
    let mut elements: Vec<i64> = Vec::with_capacity(i32::MAX as usize);

    for number in i32::MIN..=i32::MAX {
      elements.push(number as i64);
    }

    assert_eq!(binary_search(&elements, &-2_147_483_648), Some(0));
    assert_eq!(
      elements[binary_search(&elements, &-2_147_483_648).unwrap() as usize],
      -2_147_483_648
    );

    assert_eq!(
      binary_search(&elements, &2_147_483_647),
      Some(4_294_967_295)
    );
    assert_eq!(
      elements[binary_search(&elements, &2_147_483_647).unwrap() as usize],
      2_147_483_647
    );

    assert_eq!(binary_search(&elements, &-32_769), None);
    assert_eq!(binary_search(&elements, &-32_780), None);

    assert_eq!(binary_search(&elements, &32_769), None);
    assert_eq!(binary_search(&elements, &32_780), None);
  }
}
