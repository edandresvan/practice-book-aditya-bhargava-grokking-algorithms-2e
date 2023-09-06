def quicksort(array):
  if len(array) < 2:
    return array
  else:
    pivot = array[0]
    less = [i for i in array[1:] if i <= pivot]

    greater = [i for i in array[1:] if i > pivot]

    return quicksort(less) + [pivot] + quicksort(greater)


def main():
  unsorted_list = [10, 5, 2, 3, -1]
  sorted_list = quicksort(unsorted_list)
  print(sorted_list)

if __name__ == "__main__":
  main()
