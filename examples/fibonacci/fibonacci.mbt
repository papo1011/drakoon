///|
fn fibonacci_iterative(n : Int) -> Int {
  if n == 0 {
    return 0
  }
  if n == 1 {
    return 1
  }
  let mut a : Int = 0
  let mut b : Int = 1
  for i = 2; i <= n; i = i + 1 {
    let next = a + b
    a = b
    b = next
  }
  return b
}

///|
fn main {
  let input : Int = 10
  let output : Int = fibonacci_iterative(input)
  println("Fibonacci")
  println("Input:")
  println(input)
  println("Output:")
  println(output)
}
