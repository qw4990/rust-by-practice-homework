
pub fn variables_1() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}
pub fn variables_2() {
    let mut x = 1;
    x += 2;
    assert_eq!(x, 3);
    println!("Success!");
}

pub fn variables_3() {
    let x: i32 = 10;
    let yy: i32;
    {
        let _y: i32 = 5;
        yy = _y;
        println!("Inner scope value of x is {} and value of y is {}", x, _y);
    }
    println!("Outer scope value of x is {} and value of y is {}", x, yy);
}

pub fn variables_4() {
    let x = variables_4_define_x();
    println!("{}, world", x);
}
fn variables_4_define_x() -> String {
    let x = "hello";
    return x.to_string()
}

pub fn variables_5() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }
    assert_eq!(x, 5);
    let x = 42;
    println!("{}", x); // Prints "42".
}

pub fn variables_6() {
    // Remove a line in the code to make it compile
    let mut x: i32 = 1;
    println!("{}", x);
    x = 7;
    // Shadowing and re-binding
    // let x = x;
    x += 3;
    println!("{}", x);


    let _y = 4;
    // Shadowing
    let _y = "I can also be bound to text!";

    println!("Success!");
}

pub fn variables_7() {
    // fix the warning without removing x=1
    let _x = 1;
}

pub fn variables_8() {
    // Fix the error below with least amount of modification
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

pub fn variables_9() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3, 2]);

    println!("Success!");
}