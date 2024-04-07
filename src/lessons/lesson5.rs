enum Direction {
    East,
    West,
    North,
    South,
}
pub(crate) fn lesson5() {
    println!("\n lesson5 : 模式匹配");

    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    };

    let ddd = Direction::East;
    let ccc = match ddd {
        Direction::West => "abcd",
        Direction::South => "fdfff",
        _ => "efgh",
    };
    println!("ccc = {}", ccc);
}