#[test]
pub fn task1() {

// Remove something to make it work

    let x: i32 = 5;
    let mut y: i32 = 5;

    y = x;

    let z = 10; // Type of z ?

    println!("Success!");
}
#[test]
pub fn task2() {

// Fill the blank

    let v: u16 = 38_u8 as u16;

    println!("Success!");
}
#[test]
pub fn task3() {

// Modify `assert_eq!` to make it work
    let x:u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
#[test]
pub fn task4() {

// Fill the blanks to make it work

    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}
#[test]
pub fn task5() {

// Fix errors and panics to make it work

   let v1 = 251_u16 + 8;
   let v2 = i16::checked_add(251, 8).unwrap();
   println!("{},{}",v1,v2);
}

pub fn task6() {

// Modify `assert!` to make it work

    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success!");
}
#[test]
pub fn task7() {

// Fill the blank to make it work

    let x = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

fn type_of_variable<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[test]
pub fn task8() {

    assert!(0.1_f32 +0.2_f32==0.3_f32);

    println!("Success!");
// 2 варіант

     assert!(0.1 as f32 +0.2 as f32 ==0.3 as f32);

    println!("Success!");
}
#[test]
pub fn task9() {
//Two goals: 1. Modify assert! to make it work 2. Make println! output: 97 - 122
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert_eq!(sum, -5);

    for c in 'a'..='z' {
        println!("{}",c as u8);
    }
}
#[test]
pub fn task10() {

// Fill the blanks
use std::ops::{Range, RangeInclusive};
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}
#[test]
pub fn task11() {

// Fill the blanks and fix the errors

    // Integer addition
    assert!(1u32 + 2u32 == 3u32);

    // Integer subtraction
    assert!(1i32 - 2i32 == -1i32);
    assert!(1i8 - 2i8 == -1i8);

    assert!(3 * 50 == 150);

    assert!(9.6 as f32 / 3.2 as f32 == 3.0 as f32); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

