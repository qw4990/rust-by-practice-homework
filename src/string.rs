pub fn string_4() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";
    println!("{}", s);
}

pub fn string_5() {
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

pub fn string_6() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}

pub fn string_7() {
    let s = "hello, world";
    string_7_greetings(String::from(s))
}

fn string_7_greetings(s: String) {
    println!("{}", s)
}

