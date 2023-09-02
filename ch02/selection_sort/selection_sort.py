def find_smallest(arr):
  smallest = arr[0]
  smallest_index = 0

  for i in range(1, len(arr)):
    if arr[i] < smallest:
      smallest = arr[i]
      smallest_index = i

  return smallest_index


def selection_sort(arr):
  new_arr = []
  copied_arr = list(arr)

  for i in range(len(copied_arr)):
    smallest = find_smallest(copied_arr)
    new_arr.append(copied_arr.pop(smallest))

  return new_arr


def main():
  my_list = [5, 3, 6, 2, 2, -1, 10]

  print(f"Original array: {my_list}")
  print(f"Sorted array:   {selection_sort(my_list)}")
  print(f"Original array: {my_list}")


main()
