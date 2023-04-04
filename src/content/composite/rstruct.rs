#![allow(unused)]

//把结构体中具有所有权的字段转移出去后，将无法再访问该字段，但是可以正常访问其它的字段。
#[derive(Debug)]
pub struct User {
    pub(crate) active: bool,
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) sign_in_count: u64,

}

/*
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
*/
pub fn new_user() -> User {
    User {
        email: String::from("init"),
        username: String::from("init"),
        active: true,
        sign_in_count: 1,
    }
}

pub fn update(new_user: User) -> User {
    return User {
        email: String::from("another@example.com"),
        ..new_user //.. 语法表明凡是我们没有显式声明的字段，全部从 user1 中自动获取
    };

// Copy 特征：实现了 Copy 特征的类型无需所有权转移，可以直接在赋值时进行 数据拷贝，
// 其中 bool 和 u64 类型就实现了 Copy 特征，因此 active 和 sign_in_count 字段在赋值给 user2 时，  仅仅发生了拷贝，而不是所有权转移。

// 值得注意的是：username 所有权被转移给了 user2，导致了 user1 无法再被使用，但是并不代表 user1 内部的其它字段不能被继续使用
}

pub fn user_test() {
    let old_user = new_user();
    println!("{:#?}", old_user); //error


    let user = update(old_user);

    println!("{}", user.username);
    println!("{:?}", user.email);
}
//  枚举类型是一个类型，它会包含所有可能的枚举成员, 而枚举值是该类型中的具体某个成员的实例。
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Rust 吸取了众多教训，决定抛弃 null，而改为使用 Option 枚举变量来表述这种结果。
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
