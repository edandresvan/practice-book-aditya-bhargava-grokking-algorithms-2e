def factorial(x):
  if x == 1 or x == 0:
    return 1
  else:
    return x * factorial( x - 1)
  
def main():
  print(f"factorial(0) = {factorial(0)}")
  print(f"factorial(1) = {factorial(1)}")
  print(f"factorial(3) = {factorial(3)}")
  print(f"factorial(5) = {factorial(5)}")
  print(f"factorial(8) = {factorial(8)}")
  print(f"factorial(15) = {factorial(15)}")
  print(f"factorial(20) = {factorial(20)}")

main()
