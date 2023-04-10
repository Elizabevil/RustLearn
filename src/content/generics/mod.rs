mod emue;
mod rcont;
mod rtrait;


//泛型损失了编译速度和增大了最终生成文件的大小

fn add<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}

// 寻找最大值
fn largest<T>(list: &[T]) -> T {
    if list.len() <= 0 {
        return T;
    }
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
