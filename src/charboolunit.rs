use std::mem::size_of_val;

pub fn charboolunit_1() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4);

    println!("Success!");
}

pub fn charboolunit_2() {
    let c1 = '中';
    charboolunit_2_print_char(c1);
}

fn charboolunit_2_print_char(c : char) {
    println!("{}", c);
}

pub fn charboolunit_5() {
    let _v: () = ();

    let v = ();
    assert_eq!(v, charboolunit_5_implicitly_ret_unit());

    println!("Success!");
}

fn charboolunit_5_implicitly_ret_unit() {
    println!("I will return a ()");
}
