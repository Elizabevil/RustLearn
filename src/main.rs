
mod ownership;
mod slice;
mod date_type;


fn main() {
    shield();
}

 fn basic() {
    println!("Hello, world!");
    let x = 5;
    let mut y = 5;
    println!("The value of x is: {}", y);
    // x = 6; //不可改变
    y = 6; //可改变
    println!("The value of x is: {}", y);

    /*-------------*/

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;    //常量
    println!("{}", THREE_HOURS_IN_SECONDS);
}

/* 变量屏蔽： 使用同一变量名代替 多种变量*/
fn shield() {
    let x = 5;
    println!("shield:{}", x);
    let x = x + 1;
    println!("shield:{}", x);

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
    println!("=================");
    let spaces = "  ";
    let spaces = spaces.len();
    println!("spaces.len(): {}", spaces); //使用同一个变量名存储不同类型数据
}
