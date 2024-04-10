//https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions
//Book 1.
// fn main() {
//     let s = String::from("hello");  // s comes into scope

//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here

//     let x = 5;                      // x comes into scope

//     makes_copy(x);                  // x would move into the function,
//                                     // but i32 is Copy, so it's okay to still
//                                     // use x afterward

// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.

//Book 2.
// fn main() {
//     let s1 = gives_ownership();         // gives_ownership moves its return
//                                         // value into s1

//     let s2 = String::from("hello");     // s2 comes into scope

//     let s3 = takes_and_gives_back(s2);  // s2 is moved into
//                                         // takes_and_gives_back, which also
//                                         // moves its return value into s3
// } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
//   // happens. s1 goes out of scope and is dropped.

// fn gives_ownership() -> String {             // gives_ownership will move its
//                                              // return value into the function
//                                              // that calls it

//     let some_string = String::from("yours"); // some_string comes into scope

//     some_string                              // some_string is returned and
//                                              // moves out to the calling
//                                              // function
// }

// // This function takes a String and returns one
// fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
//                                                       // scope

//     a_string  // a_string is returned and moves out to the calling function
// }

// Practices
// 1.
// fn main() {
//     let x = String::from("hello, world");
//     let y = x.clone();
//     println!("{},{}",x,y);
// }

//2.
// fn main() {
//     let s1 = String::from("hello, world");
//     let s2 = take_ownership(s1);

//     println!("{}", s2);
// }

// fn take_ownership(s: String)-> String {
//     println!("{}",s);
//     s
// }

//3.
// fn main() {
//     let s: String = gives_ownership();
//     println!("{}",s);
// }

// fn gives_ownership() -> String {
//     let s: String = String::from("hello, world");
//     // let _s = s.as_bytes();
//     let _s = s.clone().into_bytes();
//     s
// }

//4.
// fn main() {
//     let s: String = String::from("hello, world");

//     print_str(s.clone());

//     println!("{}", s);
// }

// fn print_str(s: String) {
//     println!("{}",s)
// }

//5.
// fn main() {
//     let x: (i32, i32,(), &str) = (1,2,(),"hello");
//     let y: (i32, i32,(), &str) = x;
//     println!("{:?}, {:?}", x, y);
// }

//Mutability 

//6.
// fn main() {
//     let s:String = String::from("hello, ");

//     let mut s1 = s;

//     s1.push_str("world");

//     println!("Success!: s1:{}",s1)
// }

//7.
// fn main() {
//     // Box Integers get allocated to the heap
//     let x: Box<i32> = Box::new(5);

//     let mut y: Box<i32> = Box::new(1);

//     // Dereference using the star opperator
//     // access value that was allocated on the heap
//     // and assign a new value
//     *y = 4;

//     assert_eq!(*x, 5);

//     println!("Success!");
// }


//Partial Move

//Example
fn main(){
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    //`name` is moved out of person, but `age` is referenced
    let Person {name, ref age} = person;

    println!("The person's age is {}",age);
    println!("The person's name is {}",name);

    //The code below will work because it was referenced
    println!("The person's age from person struct is {}", person.age);

    //The two lines of code will not work because name was not referenced
    //println!("The person's name from person struct is {}", person.name);
    //println!("The person struct is {:?}", person);
}

