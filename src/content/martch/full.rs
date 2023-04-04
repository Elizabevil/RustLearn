pub fn asds() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // 全新的变量 y
        _ => println!("Default case, x = {:?}", x),
    }
    //一旦 match 表达式执行完毕，其作用域也就结束了，同理内部 y 的作用域也结束了
    println!("at the end: x = {:?}, y = {:?}", x, y);
}
