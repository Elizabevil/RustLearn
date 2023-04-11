mod emue;
mod rcont;
mod rtrait;


//泛型损失了编译速度和增大了最终生成文件的大小

fn add<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}
