pub fn ownerships_1() {
    // Use as many approaches as you can to make it work
    let x = String::from("Hello world");
    let y = &x;
    println!("{}, {}", x, y);
}

// Don't modify code in main!
pub fn ownerships_2() {
    let s1 = String::from("Hello world");
    let s2 = ownerships_2_take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn ownerships_2_take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

pub fn ownerships_3() {
    let s = ownership_3_give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn ownership_3_give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.into_bytes();
    String::from_utf8(_s).unwrap()
}

// Fix the error without removing any code
pub fn ownerships_4() {
    let s = String::from("Hello World");

    ownership_4_print_str(&s);

    println!("{}", s);
}

fn ownership_4_print_str(s: &String)  {
    println!("{}",s)
}

// Don't use clone ,use copy instead
pub fn ownerships_5() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}

// make the necessary variable mutable
pub fn ownerships_6() {
    let s = String::from("Hello ");

    let mut s1 = s;

    s1.push_str("World!");

    println!("Success!");
}

pub fn ownerships_7() {
    let x = Box::new(5);

    let mut y = Box::new(0);      // update this line, don't change other lines!

    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}

pub fn ownerships_8() {
    let t = (String::from("hello"), String::from("world"));

    let _s = t.0;

    // Modify this line only, don't use `_s`
    println!("{:?}", t.1);
}


pub fn ownerships_9() {
    let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (s1, s2) = (&t.0, &t.1);

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}