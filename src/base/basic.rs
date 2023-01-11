pub mod base {
    pub mod government {
        pub fn mutable_v() {
            let mut a = 123;
            a = 456;

            let b = 123;   // 可以编译，但可能有警告，因为该变量没有被使用
            let b = 456; //重新定义（换内容不换名）


            let x = 5;
            let x = x + 1;
            let x = x * 2;
            println!("The value of x is: {}", x);
        }

        pub fn date_type() {
            // addition
            let sum = 5 + 10;

            // subtraction
            let difference = 95.5 - 4.3;

            // multiplication
            let product = 4 * 30;

            // division
            let quotient = 56.7 / 32.2;
            let floored = 2 / 3; // Results in 0

            // remainder
            let remainder = 43 % 5;
        }

        pub fn basic() {
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
        pub fn shield() {
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
    }
}
 