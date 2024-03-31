//1.
// char data types can hold 4 bytes
// char data types use single quotes; NOT DOUBLE QUOTES
use std::mem::size_of_val;
fn main() {
    let c1: char = 'a';
    assert_eq!(size_of_val(&c1),4);
    
    println!("Size is {}",size_of_val(&c1));
}

//2. 
// fn main() {
//     let c1: char = 'a';
//     print_char(c1);
// }

// fn print_char(c:char){
//     println!("{}",c);
// }

//3.
// fn main() {
//     let _f: bool = false;
//     let t: bool = true;

//     if t {
//         println!("Success");
//     }
// }

//4.
// fn main() {
//     let f = true;
//     let t =  false && false;
//     assert_eq!(t,f);

//     println!("Success");
// }

//Unit Type
//5.
// fn main(){
//     let _v: () = ();

//     let v: (i32, i32) = (2,3);
//     assert_eq!(_v,implicitly_ret_unit());

//     println!("Success")

// }

// fn implicitly_ret_unit() {
//     println!("I will return a ()");
// }

// fn explicitly_ret_unit() -> () {
//     println!("I will return a ()");
// }

//6.
// Unit types have a size of 0 
// use std::mem::size_of_val;
// fn main(){
//     let unit: () = ();
//     assert!(size_of_val(&unit) == 0);

//     println!("Success");
// }