fn create_array() {
    //rust
    // 基本类型：数组的元素类型要统一，长度要固定。
    let a = [1, 2, 3, 4, 5];
    let array: [String; 8] = core::array::from_fn(|i| String::from("rust is good!"));
    println!("{:#?}", array);
}

pub fn array_test() {

    // 编译器自动推导出one的类型
    let one = [1, 2, 3];
    // 显式类型标注
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [1; 3];//默认值初始化
    let blank2: [u8; 3] = [0; 3];
    // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]

    // [u8; 3]和[u8; 4]是不同的类型，数组的长度也是类型的一部分
    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];

    // 借用arrays的元素用作循环中
    for a in &arrays {
        print!("{:?}: ", a);
        // 将a变成一个迭代器，用于循环
        // 你也可以直接用for n in a {}来进行循环
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }
}
