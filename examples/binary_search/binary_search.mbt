///|
fn binary_search(arr : FixedArray[Int], len : Int, target : Int) -> Bool {
  let mut ret = true
  let mut low = 0
  let mut high = len - 1
  for ; low <= high; {
    let median = (low + high) / 2
    if arr[median] < target {
      low = median + 1
    } else {
      high = median - 1
    }
  }
  if low == len {
    ret = false
  }
  if arr[low] != target {
    ret = false
  }
  return ret
}

///|
fn main {
  let sorted : FixedArray[Int] = [8, 22, 37, 55, 56, 102, 115]
  if binary_search(sorted, 7, 55) {
    println("Found 55")
  } else {
    println("55 not found")
  }
  if binary_search(sorted, 7, 100) {
    println("Found 100")
  } else {
    println("100 not found")
  }
}
