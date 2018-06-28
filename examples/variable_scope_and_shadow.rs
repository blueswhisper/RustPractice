fn main() {
    // 此绑定存在于main函数中
    let long_lived_binding = 1;

    {
        // 此绑定存在于最近的代码块中
        let short_lived_binding = 2;
        println!("inner short: {:?}", short_lived_binding);

        // 此绑定*隐藏*了外面的绑定
        let long_lived_binding = 5u32;
        println!("inner long: {:?}", long_lived_binding);
    }

    //println!("outer short: {:?}", short_lived_binding);

    println!("outer long: {:?}", long_lived_binding);

    // 这里应该是重新绑定
    let long_lived_binding = 'a';
    println!("outer long: {}", long_lived_binding);
}
