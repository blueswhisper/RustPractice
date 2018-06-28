fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // 将`an_integer`的值拷贝一份,并与copied_integer绑定
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // 编译器会对未使用变量绑定产生警告；可在变量名加上下划线的前缀来消除这些警告内容。
    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32;
}
