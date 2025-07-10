pub fn refborrow_1() {
    let x = 5;
    // Fill the blank
    let p = &x;

    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}

pub fn refborrow_2() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y);

    println!("Success!");
}

// Fix error
pub fn refborrow_3() {
    let s = String::from("hello, ");

    refborrow_3_borrow_object(&s);

    println!("Success!");
}

fn refborrow_3_borrow_object(_s: &String) {}

pub fn refborrow_4() {
    let mut s = String::from("hello, ");

    refborrow_4_push_str(&mut s);

    println!("Success!");
}

fn refborrow_4_push_str(s: &mut String) {
    s.push_str("world")
}

pub fn refborrow_5() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = &mut s;

    p.push_str("world");

    println!("Success!");
}

pub fn refborrow_6() {
    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let r2 = &c;

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(refborrow_6_get_addr(r1),refborrow_6_get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn refborrow_6_get_addr(r: &char) -> String {
    format!("{:p}", r)
}

// Remove something to make it work
// Don't remove a whole line !
pub fn refborrow_7() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}

pub fn refborrow_8() {
    // Fix error by modifying this line
    let mut s = String::from("hello, ");

    refborrow_8_borrow_object(&mut s);

    println!("Success!");
}

fn refborrow_8_borrow_object(_s: &mut String) {}

// This code has no errors!
pub fn refborrow_9() {
    let mut s = String::from("hello, ");

    refborrow_9_borrow_object(&s);

    s.push_str("world");

    println!("Success!");
}

fn refborrow_9_borrow_object(_s: &String) {}

// Comment one line to make it work
pub fn refborrow_10() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");

    // println!("{}",r1);
}

pub fn refborrow_11() {
    let mut s = String::from("hello, ");

    let _r1 = &mut s;
    let _r2 = &mut s;
    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time
    // println!("{}", r1);
}