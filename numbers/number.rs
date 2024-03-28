// 1.
fn main() {
    let x: i32 = 5;
    let mut _y = 5;
    _y = x;
    let _z = 10;
    println!("Success!{}",x)
}

// 2.
// fn main(){
//     let v: u16 = 38_u8 as u16;
//     println!("Success!{}",v)
// }

//3.
// fn type_of<T>(_:&T) -> String{
//     format!("{}",std::any::type_name::<T>())
// }
// fn main(){
//     let x: i32 = 5;
//     assert_eq!("i32".to_string(),type_of(&x));

//     println!("Success! {}",x)
// }

//4.
// fn main(){
//     assert_eq!(i8::MAX,127);
//     assert_eq!(u8::MAX,255);

//     println!("Success!");
// }

//5.
// fn main(){
//     let v1 = 251_u16 + 8_u16;
//     let v2 = i16::checked_add(251,8).unwrap();
//     println!("{},{}",v1,v2)
// }

//6.
// fn main(){
//     let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255
//     assert!(v == 1597);

//     println!("Success {}", v);
// }

//7.
// fn type_of<T>(_:&T) -> String{
//     format!("{}",std::any::type_name::<T>())
// }

// fn main(){
//     let x = 1_000.000_1;
//     let y: f32 = 0.12;
//     let z = 0.01_f64;

//     assert_eq!(type_of(&x), "f64".to_string());
//     assert_eq!(type_of(&y), "f32".to_string());
//     assert_eq!(type_of(&z), "f64".to_string());
//     println!("Success");
// }

//8.
// fn main() {
//     assert!(0.1_f32+0.2_f32==0.3_f32);
//     assert!(0.1 as f32 + 0.2 as f32 == 0.3 as f32);

//     println!("Success!");
// }

//9.
// fn main() {
//     let mut sum = 0;
//     for i in -3..2{
//         sum += i
//     }
//     assert!(sum == -5);
//     for c in 'a'..='z' {
//         println!("{}",c as u8)
//     }
// }

//10.
// use std::ops::{Range, RangeInclusive};
// fn main() {
//     assert_eq!((1..5), Range{start:1,end:5});
//     assert_eq!((1..=5), RangeInclusive::new(1,5));

//     println!("Success!");

// }
