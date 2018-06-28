use std::mem;

// 此函数借用一个 slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // 固定大小的数组（类型标记是多余的）
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // 所有元素可以初始化成相同的值
    let ys: [i32; 500] = [0; 500];

    // 索引从 0 开始
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` 返回数组的大小
    println!("array size: {}", xs.len());

    // 数组是在堆中分配
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // 数组可以自动地借用成为 slice
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // slice 可以指向数组的一部分
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    // 越界的索引会引发 panic（中文意思是：惊恐、恐慌等意）
    //println!("{}", xs[5]);

    // 测试自定义对象的在数组中的内存分布
    // 测试结果像是数组存储了对象所有的属性的大小
    let matrixs: [Matrix; 2] = [Matrix(1.0, 2.0, 3.0, 4.0), Matrix(5.0, 6.0, 7.0, 8.0)];
    println!(
        "two elements Matrix array occupies {} bytes",
        mem::size_of_val(&matrixs)
    );
}
