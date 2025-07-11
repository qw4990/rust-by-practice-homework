// Fill in the blanks to make it work
struct A; // Concrete type `A`.
struct S(A); // Concrete type `S`.
struct SGen<T>(T); // Generic type `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}
pub fn generics_1() {
    // Using the non-generic functions
    reg_fn(S(A)); // Concrete type.
    gen_spec_t(SGen(A)); // Implicitly specified type parameter `A`.
    gen_spec_i32(SGen(0)); // Implicitly specified type parameter `i32`.

    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('a'));

    // Implicitly specified type parameter `char` to `generic()`.
    generic(SGen('a'));

    println!("Success!");
}

fn generics_2_sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

pub fn generics_2() {
    assert_eq!(5, generics_2_sum(2i8, 3i8));
    assert_eq!(50, generics_2_sum(20, 30));
    assert_eq!(2.46, generics_2_sum(1.23, 1.23));

    println!("Success!");
}

struct generics_3_Point<T> {
    x: T,
    y: T,
}

pub fn generics_3() {
    let integer = generics_3_Point { x: 5, y: 10 };
    let float = generics_3_Point { x: 1.0, y: 4.0 };

    println!("Success!");
}

// Modify this struct to make the code work
struct generics_4_Point<T1, T2> {
    x: T1,
    y: T2,
}

pub fn generics_4() {
    // DON'T modify this code.
    let p = generics_4_Point {
        x: 5,
        y: "hello".to_string(),
    };

    println!("Success!");
}

// Add generic for Val to make the code work, DON'T modify the code in `main`.
struct generics_5_Val<T> {
    val: T,
}

impl<T> generics_5_Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

pub fn generics_5() {
    let x = generics_5_Val { val: 3.0 };
    let y = generics_5_Val {
        val: "hello".to_string(),
    };
    println!("{}, {}", x.value(), y.value());
}

struct generics_6_Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> generics_6_Point<T, U> {
    // Implement mixup to make it work, DON'T modify other code.
    fn mixup<V, W>(self, other: generics_6_Point<V, W>) -> generics_6_Point<T, W> {
        generics_6_Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn generics_6() {
    let p1 = generics_6_Point { x: 5, y: 10 };
    let p2 = generics_6_Point {
        x: "Hello",
        y: '中',
    };

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    println!("Success!");
}

// Fix the errors to make the code work.
struct generics_7_Point<T> {
    x: T,
    y: T,
}

impl generics_7_Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn generics_7() {
    let p = generics_7_Point{x: 5.0, y: 10.0};
    println!("{}",p.distance_from_origin());
}