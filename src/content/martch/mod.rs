pub mod mode;
pub mod full;

pub enum Direction {
    East,
    West,
    North,
    South,
}

pub fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    };
}


pub fn value_in_direction(direction: Direction) -> u8 {
    let ip_str = match direction {
        Direction::North => {
            println!("Lucky penny!");
            1
        },
        Direction::East => 5,
        Direction::South => 10,
        Direction::West => 25,
    };
    ip_str
}

// match target {
//     模式1 => 表达式1,
//     模式2 => {
//         语句1;
//         语句2;
//         表达式2
//     },
//     _ => 表达式3
// }


enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

pub fn action_test() {
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    /*    只匹配一项，其余直接忽略
        if let Some(3) = v {
            println!("three");
        }

        let v = Some(3u8);
        match v {
            Some(3) => println!("three"),
            _ => (),
        }*/

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
}
