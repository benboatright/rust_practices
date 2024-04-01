//1.
// fn main() {
//     let (x,y) = (1,2);
//     let s: i32 = sum(x,y);

//     assert_eq!(s,3);

//     println!("Success!");
// }

// fn sum(x: i32,y: i32) -> i32 {
//     x+y
// }

//2.
// fn main() {
//     print();
// }

// fn print() -> () {
//     println!("Success!")
// }

//3.
// // Don't let println! work
// fn main() {
//     never_return();

//     println!("Failed!");
// }

// //Diverging function
// fn never_return() -> ! {
//     // Implement this function, don't modify the fn signatures
//     panic!()

// }

//4.
// fn main(){
//     println!("success!")
// }

// fn get_option(tp: u8) -> Option<i32> {
//     // Match is like a switch block
//     // If tp is 1, the first block is executed
//     // If tp is anything other than 1, the second block is executed
//     match tp {
//         1 => {
//             // TODO
//         }
//         _ => {
//             // TODO
//         }
//     };

//     //Rather than returning a None, we use a diverging function instead
//     never_return_fn()
// }

// // Implement this in Three ways
// fn never_return_fn() -> ! {
//     //1. 
//     panic!();
//     //2.
//     //unimplemented!();
//     //3.
//     //todo!();
// }

//5.
fn main() {
    let b = false;

    let _v = match b {
        true => 1,
        //Diverging functions can also be used in match expression to replace a value
        false => {
            println!("Success!");
            panic!("we have no value for 'false', but we can panic ");
        }
    };

    println!("Exercise Failed if printing out this line!")
}
