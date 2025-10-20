///|
fn while_count(n : Int) -> Int {
  let mut i = 0
  while i < n {
    println(i)
    i = i + 1
  }
  return i
}

///|
fn sum_for(n : Int) -> Int {
  let mut s = 0
  for i = 0; i < n; i = i + 1 {
    s = s + i
  }
  return s
}

///|
fn first_over_three() -> Int {
  for i = 0; ; i = i + 1 {
    if i > 3 {
      return i
    }
  }
}

///|
fn for_only_cond(limit : Int) -> Int {
  let mut i = 0
  // Only condition; step done inside the body
  for ; i < limit; {
    println(i)
    i = i + 1
  }
  return i
}

///|
fn zero_iterations(n : Int) -> Unit {
  let i = n
  // While with false condition at entry
  while i < 0 {
    println("this should not print (while)")
  }

  // For whose condition is false at first check
  for j = 0; j < 0; j = j + 1 {
    println("this should not print (for)")
  }
}

///|
fn nested_loops() -> Unit {
  for i = 0; i < 2; i = i + 1 {
    let mut j = 0
    while j < 2 {
      println("i:")
      println(i)
      println("j:")
      println(j)
      j = j + 1
    }
  }
}

///|
fn main {
  println("while_count(3) ->")
  println(while_count(3))
  println("sum_for(5) ->")
  println(sum_for(5))
  println("first_over_three() ->")
  println(first_over_three())
  println("for_only_cond(4) ->")
  println(for_only_cond(4))
  println("zero_iterations(1) ->")
  zero_iterations(1)
  println("nested_loops() ->")
  nested_loops()
}
