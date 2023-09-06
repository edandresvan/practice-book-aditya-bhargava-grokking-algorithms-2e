pub fn quicksort_8_bits(elements: &Vec<i8>) -> Vec<i8> {
  if elements.len() < 2 {
    elements.to_vec()
  } else {
    let pivot: &i8 = &elements[0];
    let lesser = elements
      .iter()
      .filter(|&x| x < pivot)
      .copied()
      .collect::<Vec<_>>();
    let greater = elements
      .iter()
      .filter(|&x| x > pivot)
      .copied()
      .collect::<Vec<_>>();
    [
      quicksort_8_bits(&lesser),
      vec![*pivot],
      quicksort_8_bits(&greater),
    ]
    .concat()
  }
}

