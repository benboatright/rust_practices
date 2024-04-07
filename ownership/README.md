# Notes from Arfan Zubi's course
https://www.youtube.com/watch?v=BpPEoZW5IiY&t=59s

Following Rust By Practice 
https://practice.course.rs/variables.html

## Ownership
- Set of rules that govern memory management
- Enforced at compile time
- If any rules are violated, the program fails to compile

### Three Rules of Ownership
1. Each value in Rust has an owner
2. There can only be owner at a time
3. When the owner goes out of scope, the value will be dropped
- Owner: The owner of a value is the variable or data structure that holds it and is responsible for allocating and freeing the memory used to store that data

## Scope
- Range within a program for which an item is valid
- **Global**: Accessible throughout the entire program
- **Local**: Accessible only within particular functions or blocks of code (not accessible outside of that function or block)
- **General Rule**: scope ends where block of code ends 

```
{                   //1. s is not valid here
    let s = "h1";   //2. s is valid here from here
    // Do Stuff     
}                   //3. scope is over and s is invalid
```

## Memory
- A component in a computer to store data and instructions for the processor to execute
    - Random Access Memory (RAM) is volatile (i.e., when power is off all contents are lost).
    - Two types of RAM used by a program at runtime:
        1. <u>Stack Memory</u>
            - Last in, first out
            - All data stored on the stack must have a known, fixed size (all data types studied)
            - Pushing to stack is faster than allocating on the heap, because the location for new data is always at the top of the stack
            - ex. integer, float, char, etc.
            ```
            fn main(){
                let x = 42;
                let y = 10;
                let z = add_numbers(x,y);

                println!("The result is {}",z);
            }

            fn add_numbers(a: i32, b: i32) -> i32 {
                let c = a + b;
                c
            }
            ```
            - main() and it's local variables pushed on stack first
            - add_numbers() and it's local variables pushed on top of main() in the stack
            - the program then operates everything on the stack from top to bottom
        2. <u>Heap Memory</u>
            - Types of unknown size will get allocated to the heap and a pointer tot he value is pushed to the stack, because a pointer is fixed size (usize)
            - Allocating on the heap is slower than pushing to the stack
            - Accessing data on the heap is also slower, as it has to be accessed using a pointer which points to an address
            - ex. string type
            ```
            let s1 = String::from("hello");
            ```
            - s1 holds a pointer to data stored on the heap
            - s1 has a data size of 5 bytes and therefore a capacity of 5 bytes

            - s1 pints to index 0 in heap and uses its length to know how many more values to retrieve.

            s1

            |name    | value              |  
            |--------|--------------------|
            |pointer |--->  to index 0    |
            |len     |5                   |
            |capacity|5                   |
        
            
            Stored in Heap

            |index | value    |  
            |------|----------|
            |0     |h         |
            |1     |e         |
            |2     |l         |
            |3     |l         | 
            |4     |o         |

### Copy vs. Move
- Scalar values with fixed sizes will automatically get copied in the stack because copying is cheap with these values.
- i.e., the values are different in the stack
```
let x = 5 // x is of type i32
let y = x // 5 gets copied into y (i.e., y is of type i32)
```
- Dynamically sized data only gets moved
```
let s1 = String::from("hello"); // s1 is a pointer to the heap
let s2 = s1; // the s1 pointer that is pointing at the value in the heap will get copied to s2

// s2 then becomes the owner and s1 is dropped
```
- i.e., s1 and s2 point to the same location in the heap

### Deep Copy
```
let s1 = String::from("hello"); // s1 is a pointer to the heap
let s2 = s1.clone(); // the actual data in heap memory is copied

// s2 then becomes the owner and s1 is dropped
```
## So What?
- Ownership prevents memory safety issues:
    - Dangling pointers
    - Double-free: trying to free memory that has already been freed
    - Memory leaks: not freeing memory that should have been freed