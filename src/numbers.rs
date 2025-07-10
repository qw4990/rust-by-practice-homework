pub fn numbers_1() {
    // Remove something to make it work
    let x = 5;
    let mut y: u32 = 5;

    println!("{}", y);

    y = x;

    let z = 10; // Type of z ?

    println!("Success! {} {}", y, z);
}

pub fn numbers_2() {
    let v: u16 = 38_u8 as u16;

    println!("Success! {}", v);
}

// Modify `assert_eq!` to make it work
pub fn numbers_3() {
    let x = 5;
    assert_eq!("i32".to_string(), numbers_3_type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn numbers_3_type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

pub fn numbers_4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}

pub fn numbers_5() {
    let v1 = 251_u16 + 8;
    let v2 = i16::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);
}

pub fn numbers_6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success!");
}

pub fn numbers_7() {
    let x = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64
    println!("{} {} {}", x, y, z);

    assert_eq!(numbers_7_type_of(&x), "f64".to_string());
    println!("Success!");
}

fn numbers_7_type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

pub fn numbers_8() {
    println!("{}", 0.1+0.2);
    assert!(0.1+0.2==0.30000000000000004);

    println!("Success!");
}

pub fn numbers_9() {
    let mut sum = 0;
    for i in -3..3 {
        sum += i
    }

    assert!(sum == -3);

    // for c in 'a'..='z' {
    //     println!("{}",c);
    // }
}

use std::ops::{Range};

pub fn numbers_10() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    // assert_eq!((1..6), RangeInclusive::new(1, 5));

    println!("Success!");
}

pub fn numbers_11() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    // assert!(1u8 - 2 == 255);

    assert!(3 * 50 == 150);

    // assert!(9.6 / 3.2 == 3.0); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    // println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    // println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    // println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    // println!("1 << 5 is {}", 1u32 << 5);
    // println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}