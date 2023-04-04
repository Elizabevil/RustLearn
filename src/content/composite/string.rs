pub fn slice() {
    let s = String::from("hello world");

    let hello = &s[0..5];
// world 是一个切片，该切片的指针指向 s 的第 7 个字节(索引从 0 开始, 6 是第 7 个字节)
// 且该切片的长度是 5 个字节。

    let world = &s[6..11];
}
/*

指针借用
修改借用的变量 mut 指针(可变引用)
*/
pub fn sdf() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    //s.clear() 处可变借用与不可变借用试图同时生效，因此编译无法通过。
    // s.clear(); // error! clear 需要清空改变 String，因此它需要一个可变借用
    println!("the first word is: {}", word);//使用了不可变借用
    s.clear(); // success
}

pub fn first_word(s: &String) -> &str {
    &s[..1]
}


// String 与 &str 的转换
// 在之前的代码中，已经见到好几种从 &str 类型生成 String 类型的操作：
//
// String::from("hello,world")
// "hello,world".to_string()


// 操作要修改原来的字符串，则该字符串必须是可变的，即字符串变量必须由 mut 关键字修饰。
pub fn operate() {
    //在字符串尾部可以使用 push() 方法追加字符 char，也可以使用 push_str() 方法追加字符串字面量。
    // 这两个方法都是在原有的字符串上追加，并不会返回新的字符串
    let mut s = String::from("Hello ");

    s.push_str("rust");
    println!("追加字符串 push_str() -> {}", s);
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s);

    let new_string_replace = s.replace("rust", "RUST");
    println!("replace 之后：{}", new_string_replace);
    s.remove(0);
    println!("remove 字符串 remove() -> {}", s);

    s += "as"; //相当于调用了 std::string 标准库中的 add() 方法，

    println!("remove 字符串 remove() -> {}", s);
}
