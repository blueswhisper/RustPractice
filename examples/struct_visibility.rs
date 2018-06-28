mod my {
    // 一个公有访问属性的结构体,带有一个公有的泛型字段`T`的字段
    pub struct WhiteBox<T> {
        pub contents: T,
    }

    // 一个公有访问属性的结构体,带有一个私有的泛型字段`T`的字段
    #[allow(dead_code)]
    pub struct BlackBox<T> {
        contents: T,
    }

    impl<T> BlackBox<T> {
        pub fn new(contents: T) -> BlackBox<T> {
            BlackBox { contents: contents }
        }
    }
}

fn main() {
    // 带公有字段的公有结构体,可以像平常一样构造
    let white_box = my::WhiteBox {
        contents: "public information",
    };

    // 并且它们的字段是可以正常访问到的
    println!("the WhiteBox contains: {}", white_box.contents);

    // 带有私有字段的公有结构体只能通过构造器构造
    let _black_box = my::BlackBox::new("classified information");

    // 一个结构体中的私有字段不能被直接访问
    // println!("The BlackBox contains: {}", _black_box.contents);
}
