// Fill the blanks
enum Direction {
    East,
    West,
    North,
    South,
}
pub fn matches_1() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North  => { // Matching South or North here
            println!("South or North");
        },
        _ => println!("West"),
    };
}

pub fn matches_2() {
    let boolean = true;

    // Fill the blank with a match expression:
    //
    // boolean = true => binary = 1
    // boolean = false =>  binary = 0
    let binary = {
        match boolean {
            true => 1,
            false => 0,
        }
    };

    assert_eq!(binary, 1);

    println!("Success!");
}