fn VAr() {
    // 默认不可变，而且自始至终不可变+类型推断
    let y = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;

    let _x = 5;//不要警告未使用的变量
}

struct Struct {
    e: i32,
}

fn variable_deconstruction() {
    let (a, mut b): (bool, bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);
    println!("在 Rust 1.59 版本后，我们可以在赋值语句的左式中使用元组、切片和结构体模式了");

    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}

// 常量不允许使用 mut。常量不仅仅默认不可变，而且自始至终不可变，因为常量在编译完成后，已经确定它的值。
// 常量使用 const 关键字而不是 let 关键字来声明，并且值的类型必须标注。
fn constant() {
    const MAX_POINTS: u32 = 100_000;
}

//Rust 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的，
fn shadowing() {
    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    // mut 变量的使用是不同的，第二个 let 生成了完全不同的新变量，两个变量只是恰好拥有同样的名称，涉及一次内存对象的再分配
    // 而 mut 声明的变量，可以修改同一个内存地址上的值，并不会发生内存对象的再分配，性能要更好。
    let x = x + 1; //6

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x); //12
    }

    println!("The value of x is: {}", x); //6
    println!("同一个变量名：-------但值与类型都不同--------", );

    // 字符串类型
    let spaces = "   ";
// usize数值类型
    let spaces = spaces.len();
}
