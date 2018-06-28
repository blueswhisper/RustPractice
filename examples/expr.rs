/*如果代码块最后一条表达式结尾处有分号，那 么返回值将变成 ()。
（译注：代码块中的最后一条语句是代码块中实际执行的最后一条语句，而不一 定是代码块中最后一行的语句。）*/

fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 将此表达式赋给 `y`
        x_cube + x_squared + x
    };

    let z = {
        // 分号结束了这个表达式，于是将 `()` 赋给 `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
