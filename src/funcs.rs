pub fn funcs_1() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = funcs_1_sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn funcs_1_sum(x: i32, y: i32) -> i32 {
    x + y
}

pub fn funcs_2() {
    funcs_2_print();
}

fn funcs_2_print() {
    println!("Success!");
}

