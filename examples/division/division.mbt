///|
fn division(a : Double, b : Double) -> Double {
  if b == 0.0 {
    println("ERR")
    return -1.0
  } else {
    return a / b
  }
}

///|
fn main {
  let a : Double = division(30.2, 2.0)
  println("Result:")
  println(a)
  let b : Double = division(30.2, 0.0)
  println("Result:")
  println(b)
}
