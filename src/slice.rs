fn slice_show() {
    let s = String::from("hello");

    let len = s.len();
    let slice = &s[0..2];
    let slice = &s[..2];
    let slice = &s[3..len];
    let slice = &s[3..];
}

fn slice_error() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); // error! //clear 需要清空 String，它尝试获取一个可变引用。
    // 在调用 clear 之后的 println! 使用了 word 中的引用，所以这个不可变的引用在此时必须仍然有效
    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

