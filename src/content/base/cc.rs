// 序列只允许用于数字或字符类型，原因是：它们可以连续，同时编译器在编译期可以检查该序列是否为空，字符和数字值是 Rust 中仅有的可以用于判断是否为空的类型。
pub fn range() {
    for i in 1..=5 {
        println!("{}", i);
    }

    for i in 'a'..='z' {
        println!("{}", i);
    }
}

use num::complex::Complex;

pub fn num_complex() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im)
}

// Rust 的字符不仅仅是 ASCII，所有的 Unicode 值都可以作为 Rust 字符，包括单个的中文、日文、韩文、emoji 表情符号等等，都是合法的字符类型。Unicode 值的范围从 U+0000 ~ U+D7FF 和 U+E000 ~ U+10FFFF
// 由于 Unicode 都是 4 个字节编码，因此字符类型也是占用 4 个字节：
pub fn char() {
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';

    println!("字节的内存大小:{}", std::mem::size_of_val(&heart_eyed_cat));
}

// 单元类型就是 () ，对，你没看错，就是 () ，唯一的值也是 () ，一些读者读到这里可能就不愿意了，你也太敷衍了吧，管这叫类型？
//没错， main 函数就返回这个单元类型 ()，你不能说 main 函数无返回值，因为没有返回值的函数在 Rust 中是有单独的定义的：发散函数( diverge function )，顾名思义，无法收敛的函数。
// 这种用法和 Go 语言的 struct{} 类似，可以作为一个值用来占位，但是完全不占用任何内存。


// 函数没有返回值，那么返回一个 ()
// 通过 ; 结尾的表达式返回一个 ()
pub fn report<T: std::fmt::Debug>(item: T) {
    println!("{:?}", item);
}

pub fn clear(text: &mut String) -> () {
    *text = String::from("");
}
