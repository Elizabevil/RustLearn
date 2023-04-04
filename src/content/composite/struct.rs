pub fn tup() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
    println!("The value of tup.0 is: {}", tup.0);
}


pub fn returnStr() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("returnStr:The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}
