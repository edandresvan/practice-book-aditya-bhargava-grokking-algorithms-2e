def binary_search(arr, item):

  low = 0
  high = len(arr) - 1
  while (low <= high):

    mid = low + (high - low) // 2
    guess = arr[mid]

    if (guess == item):
      return mid

    if (guess < item):
      low = mid + 1

    if (guess > item):
      high = mid - 1

  return None


def main():
  my_list = [1, 3, 5, 7, 9]
  assert binary_search(my_list, 3) == 1
  print(f"Searching index of  3: {binary_search(my_list, 3)}")

  assert binary_search(my_list, -1) is None
  print(f"Searching index of -1: {binary_search(my_list, -1)}")

  assert binary_search(my_list, 6) is None
  print(f"Searching index of  6: {binary_search(my_list, 6)}")

  assert binary_search(my_list, 7) == 3
  print(f"Searching index of  7: {binary_search(my_list, 7)}")

  assert binary_search(my_list, 11) is None
  print(f"Searching index of 11: {binary_search(my_list, 11)}")

  my_list = []
  for i in range(-32768, 32768):
    my_list.append(i)

  

  assert binary_search(my_list, 32767) is not None
  print(f"Searching index of -32_768: {binary_search(my_list, -32768)}")

  assert binary_search(my_list, 32767) is not None
  print(f"Searching index of 32_767: {binary_search(my_list, 32767)}")


  assert binary_search(my_list, -32769) is None
  print(f"Searching index of -32_769: {binary_search(my_list, -32769)}")

  assert binary_search(my_list, -32780) is None
  print(f"Searching index of -32_780: {binary_search(my_list, -32769)}")

  assert binary_search(my_list, 32769) is None
  print(f"Searching index of 32_769: {binary_search(my_list, 32769)}")

  assert binary_search(my_list, 32780) is None
  print(f"Searching index of 32_780: {binary_search(my_list, 32769)}")

  deleted_element = my_list[45204]
  my_list.remove(deleted_element)

  assert binary_search(my_list, deleted_element) is None
  print(f"Searching index of deleted {deleted_element}: {binary_search(my_list, deleted_element)}")


main()