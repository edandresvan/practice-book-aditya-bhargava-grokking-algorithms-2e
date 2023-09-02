/// Gets the index of the smallest element in the given collection.
///
/// # Parameters
///
/// * `elements` &[\[T\]](slice): Collection where the smallest item will be searched.
///
/// # Returns
///
/// * [`Some`] ([`usize`]): The index of the smallest element found in the given collection.
/// * [`None`]: If the given collection is empty.
///
pub fn find_smallest<T>(elements: &[T]) -> Option<usize>
where
  T: PartialOrd + Clone,
{
  if elements.is_empty() {
    return None;
  }

  // Assume that the first element is the smallest in the collection
  let mut smallest_index: usize = 0;

  // Look for another item that may be smaller than the current one, starting
  // with the second item in the collection.
  for (index, element) in elements.iter().enumerate().skip(1) {
    if element < &elements[smallest_index] {
      smallest_index = index;
    }
  }

  Some(smallest_index)
}

/// Creates a new vector with the sorted elements from the given collection.
///
/// # Arguments
///
/// * `elements` &[\[T\]](slice): Collection of items to be taken to create a new collection with those items already sorted.
///
/// # Returns
///
/// * [`Vec`]<[\[T\]](slice)>: A new collection with items already sorted.
///
/// # Remarks
///
/// If the given collection is empty, then a new empty collection will be
/// returned.
pub fn selection_sort<T>(elements: &[T]) -> Vec<T>
where
  T: PartialOrd + Clone,
{
  if elements.is_empty() {
    return Vec::<T>::new();
  }

  // A cloned collection with the unsorted elements.
  let mut copied_elements: Vec<T> = elements.to_vec().clone();
  // An empty collection to store the sorted elements.
  let mut sorted_elements: Vec<T> = Vec::with_capacity(elements.len());

  // Iterate as many times as items remain in the cloned collection.
  for _ in 0..copied_elements.len() {
    // Find the index of the smallest element.
    let smallest_index = find_smallest(&copied_elements).unwrap();
    // From the cloned collection removes the smallest item found.
    // And then add that item to the new ordered collection
    sorted_elements.push(copied_elements.remove(smallest_index));
  }

  sorted_elements
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
