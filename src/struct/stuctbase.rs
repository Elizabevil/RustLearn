struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 变量与字段同名时的字段初始化简写语法
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // 使用结构体更新语法从其他实例创建实例
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
