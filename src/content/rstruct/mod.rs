mod method;

impl method::Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 允许方法名跟结构体的字段名相同：
    //  impl 中且没有 self 的函数被称之为关联函数,,没有 self 是一个函数而不是方法
    // 它又在 impl 中，与结构体紧密关联，因此称为关联函数。//需要用 :: 来调用
    pub fn new(width: u32, height: u32) -> Self {
        method::Rectangle { width, height }
    }
    pub fn width(&self) -> u32 {
        return self.width;
    }
}


// self 、 & self 和 & mut self
// self 表示 Rectangle 的所有权转移到该方法中，这种形式用的较少
// &self  表示该方法对 Rectangle 的不可变借用 // &self 其实是 self: &Self
// &mut self 表示可变借用,//改变当前结构体

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}


fn main() {
    let rect1 = method::Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    let m = Message::Write(String::from("hello"));
    m.call();
}
