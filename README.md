<p align="center">
  <img src="https://francescopapini.com/assets/img/projects/drakoon.png" alt="Drakoon" width="400"/>
</p>

# Drakoon: Initial Target Support

## Data Types

### Basic Data Types
- `Int` (32-bit signed integer)
- `Double` (64-bit floating point number)
- `Unit` (unit type, similar to `void` in C/C++)

### Aggregate Data Types
- `Array` (1-dimension)

---

## Variables
```rust
let a : Int = 10 // explicit type declaration
let b = 20 // implicit type inference
let mut c = 0 // mutable variable
```

### Global Variables
- declaration + mandatory initialization

### Local Variables
- declaration + mandatory initialization

---

## Constants
- global constants (using `const`)

---

## Operators

### Arithmetic Operators
- addition (`+`)
- subtraction (`-`)
- multiplication (`*`)
- division (`/`)

### Relational Operators
- equal to (`==`)
- not equal to (`!=`)
- greater than (`>`)
- less than (`<`)
- greater than or equal to (`>=`)
- less than or equal to (`<=`)

### Bitwise Operators
- AND (`&`)
- OR (`|`)
- XOR (`^`)

### Assignment Operators
- simple assignment (`=`)

---

## Control Statements

### Decision Making
- `if` statement
- `if - else if - else` statement
- nested `if` statement

### Loops
- `for` loop
- `for` loop as infinite loop
- `while` loop 

---

## Functions

- parameter list:
  - simple variables
  - arrays
- passing:
  - by value
  - by reference
- return types:
  - `Int`
  - `Double`
  - `Unit`

---

## Output

- `println` instruction

---

## Instructions

- declaration + initialization
- `return` statement / implicit return
- function call
- expression
