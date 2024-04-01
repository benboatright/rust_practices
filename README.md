# Notes from Arfan Zubi's course
https://www.youtube.com/watch?v=BpPEoZW5IiY&t=59s

Following Rust By Practice 
https://practice.course.rs/variables.html

## Variables
- var declaration: use 'let' to declare vars
- print:  print!() println!() (println!() adds line afterwards)
- scope: defined by block of code that it was declared
- function: is a named blick of code that is reusable
- shadowing: allows a var to be re-declared in the same scope with same name

## Data Types

### Integers
- Signed: postive and negative integers
- Unsigned interger: always positive
- Arch: architecture (usually 64bit)

|Length   | Signed   | Unsigned  |
|---------|----------|-----------|
|8-bit    |i8        | u8        |
|16-bit   |i16       | u16       |
|32-bit   |i32**     | u32       |
|64-bit   |i64       | u64       |
|128-bit  |i128      | u128      |
|arch     |isize     | usize     |
**default type


### Floating Point
|Length   | type     |
|---------|----------|
|32-bit   |i32       |
|64-bit   |i64**     |
**default type

### Char
- Single character of 
- Size = 4 bytes

### Bool
- Boolean value of true or false
- Size = 1 bytes

### Unit
- Empty tuple used to return "nothing" in expressions or functions
- Size = 0 bytes

### Statement
- Instructions that perform some action but do not produce a value
- Function definitions are statements, as well as code that ends with ";" (usually)

### Expression 
- Evaluate to a resultant value

### Function