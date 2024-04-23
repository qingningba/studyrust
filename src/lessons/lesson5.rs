enum Direction {
    East,
    West,
    North,
    South,
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
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


    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1,2),
        Action::ChangeColorRGB(255,255,0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r, g, _) => {
                println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                         r, g,
                );
            }
        }
    }


    let foo = 'f';
    let bar = matches!(foo, 'A'..='Z' | 'a'..='z');
    println!("bar = {}", bar);
}