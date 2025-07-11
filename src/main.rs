use crate::closure::closure_1;

mod variables;
mod numbers;
mod charboolunit;
mod stmtfunc;
mod funcs;
mod ownership;
mod refborrow;
mod string;
mod array;
mod matches;
mod generics;
mod closure;

fn variables_() {
    variables::variables_1();
    variables::variables_2();
    variables::variables_3();
    variables::variables_4();
    variables::variables_5();
    variables::variables_6();
    variables::variables_7();
    variables::variables_8();
    variables::variables_9();
}

fn numbers_() {
    numbers::numbers_1();
    numbers::numbers_2();
    numbers::numbers_3();
    numbers::numbers_4();
    numbers::numbers_5();
    numbers::numbers_6();
    numbers::numbers_7();
    numbers::numbers_8();
    numbers::numbers_9();
    numbers::numbers_10();
    numbers::numbers_11();
}

fn charboolunit_() {
    charboolunit::charboolunit_1();
    charboolunit::charboolunit_2();
    charboolunit::charboolunit_5();
}

fn stmtfunc_() {
    stmtfunc::stmtfunc_1();
    stmtfunc::stmtfunc_2();
    stmtfunc::stmtfunc_3();
}

fn funcs_() {
    funcs::funcs_1();
    funcs::funcs_2();
}

fn ownerships_() {
    ownership::ownerships_1();
    ownership::ownerships_2();
    ownership::ownerships_3();
    ownership::ownerships_4();
    ownership::ownerships_5();
    ownership::ownerships_6();
    ownership::ownerships_7();
    ownership::ownerships_8();
    ownership::ownerships_9();
}

fn refborrow_() {
    refborrow::refborrow_1();
    refborrow::refborrow_2();
    refborrow::refborrow_3();
    refborrow::refborrow_4();
    refborrow::refborrow_5();
    refborrow::refborrow_6();
    refborrow::refborrow_7();
    refborrow::refborrow_8();
    refborrow::refborrow_9();
    refborrow::refborrow_10();
    refborrow::refborrow_11();
}

fn string_() {
    string::string_4();
    string::string_5();
    string::string_6();
    string::string_7();
}

fn array_() {
    array::array_1();
    array::array_2();
    array::array_3();
}

fn matches_() {
    matches::matches_2();
}

fn generics_() {
    generics::generics_1();
    generics::generics_2();
    generics::generics_3();
    generics::generics_4();
    generics::generics_5();
    generics::generics_6();
    generics::generics_7();
}

fn closure_() {
    closure::closure_1();
    closure::closure_2();
}

fn main() {
    variables_();
    numbers_();
    charboolunit_();
    stmtfunc_();
    funcs_();
    ownerships_();
    refborrow_();
    string_();
    array_();
    matches_();
    generics_();
    closure_();
}
