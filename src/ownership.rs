fn main() {
    /*  我们不能为了每一个在编译时大小未知的文本而将一块内存放入二进制文件中，并且它的大小还可能随着程序运行而改变。
          对于 String 类型，为了支持一个可变，可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放内容。这意味着：
              必须在运行时向内存分配器请求内存。
              需要一个当我们处理完 String 时将内存返回给分配器的方法。*/

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() 在字符串后追加字面值

    println!("{}", s); // 将打印 `hello, world!`
    // GC:内存在拥有它的变量离开作用域后就被自动释放,rust 在结尾的 } 处自动调用 drop
}

fn simple_copy() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}\n{}", s1, s2);
    s1.push_str("1");
    println!("{}\n{}", s1, s2);
}

fn mutable_references() {
    let mut s = String::from("hello");

    change(&mut s);//创建可变引用 ， 可变变量 mut + 可变引用 &mut
    // 在同一时间，只能有一个对某一特定数据的可变引用。尝试创建两个可变引用的代码将会失败：

    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dsasd() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;

    // 同时使用可变与不可变引用中 会报错
    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    let r3 = &mut s; // 大问题
    println!("{}, {}, and {}", r1, r2, r3);

    // 修复

    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);
}


// 悬垂引用（Dangling References）其指向的内存可能已经被分配给其它持有者
fn dangle() -> String { // dangle 返回一个字符串的引用

    let s = String::from("hello"); // s 是一个新字符串

    // &s // 返回字符串 s 的引用
    s //修改
} // 这里 s 离开作用域并被丢弃。其内存被释放。
// 危险！