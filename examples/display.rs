use std::fmt;

// 带有两个数字的结构体。`Debug` 将被派生，可以看到输出结果和 `Display` 的差异。
#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

// 为了比较，定义一个含有字段的结构体。
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// 类似地对 Point2D 进行实现
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 自定义方式实现，仅让 `x` 和 `y` 标识出来。
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 自定义方式实现，仅让 `x` 和 `y` 标识出来。
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    //命名占位符的使用依赖于fmt:Display trait
    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // 报错。`Debug` 和 `Display` 都被实现了，但 `{:b}` 需要 `fmt::Binary`
    // 得到实现。这语句不能运行。
    // println!("What does Point2D look like in binary: {:b}?", point);

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Compare Complex: ");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}
