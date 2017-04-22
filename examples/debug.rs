#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Deep(DebugPrintable);

fn main() {
	// 打印操作使用 `{:?}` 和使用 `{}` 类似。
	// 使用`{:?}`要求类型实现了fmt:Debug trait.
	// 所有rust提供的原生类型都已经实现了该trait.
	// 自定义类型需要自己实现。一般使用官方提供的宏
	// #[derive(Debug)]就已足够。
	// 使用`{}`要求类型实现了fmt:Display trait
	// fmt:Debug 和 fmt:Display的区别就是名字所
	// 示, fmt:Debug主要用于调试目的,不保证输出的
	// 是utf8字符串。而fmt:Display则保证忠实的使用
	// utf8字符串显示类型状态
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");
    // `DebugPrintable` 是能够打印的类型。
    println!("Now {:?} will print!", DebugPrintable(3));
    // 使用 `derive` 的一个问题是不能控制输出的形式。
    // 假如我只想展示一个 `7`？
    println!("Now {:?} will print!", Deep(DebugPrintable(7)));
}