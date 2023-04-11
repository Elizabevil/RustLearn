/*vector 允许我们在一个单独的数据结构中储存多于一个的值，它在内存中彼此相邻地排列所有的值*/

pub fn new_vector() {
    let v: Vec<i32> = Vec::new(); // new 创建并指定是数据类型 (泛型)
    let mut v = vec![1, 2, 3]; // 宏创建

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}
