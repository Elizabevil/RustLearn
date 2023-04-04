// Rust 在编译的时候会扫描代码，变量声明后未使用会以 warning 警告的形式进行提示），
// 引入 # ![allow(unused_variables)] 属性标记，该标记会告诉编译器忽略未使用的变量，
/*
返回一个! 类型，这个表明该函数是一个发散函数，不会返回任何值，包括 ()

unimplemented!() 告诉编译器该函数尚未实现，
unimplemented!() 标记通常意味着我们期望快速完成主要代码，回头再通过搜索这些标记来完成次要代码，
类似的标记还有 todo!()

只使用基本类型的局限性：无法从更高的抽象层次去简化代码。
通过 open(&mut f1) 进行调用，也远没有使用 f1.open() 来调用好
*/
pub mod string;
pub mod tup;
mod r#struct;
