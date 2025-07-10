pub fn stmtfunc_1() {
    let v = {
        let x = 1;
        x + 2
    };

    assert_eq!(v, 3);

    println!("Success!");
}

pub fn stmtfunc_2() {
    let v = {let x = 3; x };

    assert!(v == 3);

    println!("Success!");
}

pub fn stmtfunc_3() {
    let s = stmtfunc_sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn stmtfunc_sum(x: i32, y: i32) -> i32 {
    x + y
}