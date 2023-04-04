#![allow(unused)]


/*
模式一般由以下内容组合而成：
        字面值
        解构的数组、枚举、结构体或者元组
        变量
        通配符
        占位符
*/

fn sdasd() {
// Vec是动态数组
    let mut stack = Vec::new();

// 向数组尾部插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);

// stack.pop从数组尾部弹出元素
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}


fn asd() {
    let (x, y, z) = (1, 2, 3);//模式和值的类型必需相同
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    // x 也是一种模式绑定，代表将匹配的值绑定到变量 x 上。因此，在 Rust 中,变量名也是一种模式，只不过它比较朴素很不起眼罢了。
}


fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn fun() {
    let point = (3, 5);
    print_coordinates(&point);


    // 类似  let, for和match     都必须要求完全覆盖匹配，才能通过编译(不可驳模式匹配)。
    // if let Some(x) = some_option_value { // if let 只需一种
    //     println!("{}", x);
    // }

    // let Some(x) = some_option_value;
}
