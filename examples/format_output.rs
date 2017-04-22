fn main() {
	//`{}`会被任意内容替换,值会转化为字符串
    println!("{} days", 30);

    //`{}`可以指定位置参数,位置参数从0开始
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    //`{}`也可以使用赋值语句
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
    // 特殊的格式实现可以在后面加上 `:` 符号。
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 创建一个包含` I32 `类型结构体(structure)。命名为 `Structure`。
    #[allow(dead_code)]
    struct Structure(i32);
    // 但是像结构体这样自定义类型需要更复杂的方式来处理。
    // 下面语句无法运行。
    //println!("This struct `{}` won't print...", Structure(3));
}