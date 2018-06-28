use List::*;

enum List {
    // Cons： 元组结构体，包含一个元素和一个指向下一节点的指针
    Cons(u32, Box<List>),
    // Nil： 末结点，表明链表结束
    Nil,
}

impl List {
    // 创建一个空链表
    fn new() -> List {
        Nil
    }

    // 向头部节点前一个位置追加元素
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // 返回链表的长度
    fn len(&self) -> u32 {
        // `self` 必须匹配，因为这个方法的行为取决于 `self` 的变化类型
        // `self` 为 `&List` 类型，`*self` 为 `List` 类型，一个具体的 `T` 类型的匹配
        // 要参考引用 `&T` 的匹配
        match *self {
            // 不能得到 tail 的所有权，因为 `self` 是借用的；
            // 而是得到一个 tail 引用
            Cons(_, ref tail) => 1 + tail.len(),
            // 基本情形：空列表的长度为 0
            Nil => 0,
        }
    }

    // 将列表以字符串（堆分配的）的形式返回
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` 和 `print!` 类似，但返回的是一个堆分配的字符串，而不是
                // 打印结果到控制台上
                format!("{}, {}", head, tail.stringify())
            }
            Nil => format!("Nil"),
        }
    }
}

fn main() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    // 显示链表的最后状态
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
