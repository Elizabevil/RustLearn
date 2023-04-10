fn display_array<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr);
}

//重点在于 N 这个泛型参数，它是一个基于值的泛型参数！因为它用来替代的是数组的长度。
fn display_array2<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}
fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(&arr);
    let arr: [i32; 2] = [1, 2];
    display_array(&arr);

    display_array2(arr)
}
