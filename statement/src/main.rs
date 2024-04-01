//Example
// fn main() {
//     let x = 5u32;

//     // This is a statment
//     let y = {
//         let x_squared = x*x;
//         let x_cubed = x_squared * x;

//         // This value gets assigned to 'y' 
//         // becasue the semicolon is omitted
//         x_cubed + x_squared + x
//     };

//     let z = {
//         // The semicolon suppresses the expression and `()` (i.e., a unit type) is assigned to 'z'
//         2*x;
//     };
    
//     println!("x is {:?}",x);
//     println!("y is {:?}",y);
//     println!("z is {:?}",z);
// }

//1.
// fn main(){
//     let v = {
//         let mut x = 1;
//         x += 2;
//         x
//     };

//     assert_eq!(v, 3);

//     println!("Success!")
// }

//2.
// fn main(){
//     let v = {
//         let x = 3;
//         x
//     };

//     assert!(v == 3);

//     println!("Success");
// }

//3.
fn main(){
    let s:i32 = sum(1,2);
    assert_eq!(s,3);

    println!("Success");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}