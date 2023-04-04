#![allow(unused)]

fn asdd() {
    {                      // s 在这里无效，它尚未声明
        let s = "hello";   // 从此处起，s 是有效的

        // 使用 s
    }                      // 此作用域已结束，s不再有效
}

//Rust 为我们提供动态字符串类型: String

fn string() {

    // 字符串字面值是不可变的，因为被硬编码到程序代码中
    // 并非所有字符串的值都能在编写代码时得知
    let s = "hello"; //s是被硬编码进程序里的字符串值（类型为 & str ）

    let s = String::from("hello");
}

//常规引用是一个指针类型，指向了对象存储的内存地址
// 引用与解引用
fn borrowing() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}


// 指向 s1 的引用，但是并不拥有它。
fn sss() {
    let s1 = String::from("hello");
    // 引用指向的值默认也是不可变的
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
    println!("-----------------");

//声明 s 是可变类型
    let mut s = String::from("hello");

    change(&mut s);
}

//无需像上章一样：先通过函数参数传入所有权，然后再通过函数返回来传出所有权，代码更加简洁
// calculate_length 的参数 s 类型从 String 变为 &String
fn calculate_length(s: &String) -> usize {
    s.len()
}

//同一作用域，特定数据只能有一个可变引用
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//这种限制的好处就是使 Rust 在编译期就避免数据竞争，数据竞争可由以下行为造成：
//
// 两个或更多的指针同时访问同一数据
// 至少有一个指针被用来写入数据
// 没有同步数据访问的机制


//Non-Lexical Lifetimes(NLL)，专门用于找到某个引用在作用域(})结束前就不再被使用的代码位置。

//  同一作用域，特定数据只能有一个可变引用
// 可变引用与不可变引用不能同时存在
fn cite() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;

    // 注意，引用的作用域 s 从创建开始，一直持续到它最后一次使用的地方
    // 这个跟变量的作用域有所不同，变量的作用域从创建持续到某一个花括号


    let v1 = &s;
    let v2 = &s;
    println!("{} and {}", v1, v2);
    // 新编译器中，r1,r2作用域在这里结束
    //引用作用域的结束位置从花括号变成最后一次使用的位置

    let r3 = &mut s;
    println!("{}", r3);
}

// 悬垂引用(Dangling References)
// 悬垂引用也叫做悬垂指针，意思为指针指向某个值后，这个值被释放掉了，而指针仍然存在，
// 其指向的内存可能不存在任何值或已被其它变量重新使用。
// 在 Rust 中编译器可以确保引用永远也不会变成悬垂状态：
// 当你获取数据的引用后，编译器可以确保数据不会在引用结束前被释放，要想释放数据，必须先停止其引用的使用。

fn dangle() -> String { // dangle 返回一个字符串的引用
    let s = String::from("hello"); // s 是一个新字符串
    // &s //error  返回字符串 s 的引用
    s //success String 的 所有权被转移给外面的调用者
} // 这里 s 离开作用域并被丢弃。其内存被释放。
// 危险！
