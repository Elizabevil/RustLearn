//Rust 允许我们为一个结构体定义多个 impl 块

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}
//Rust 允许我们为一个结构体定义多个 impl 块

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
