fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;
    println!("Before mutation: {:?}", mutable_binding);
    mutable_binding += 1;
    println!("After mutation: {:?}", mutable_binding);

    // 错误代码
    //_immutable_binding += 1;
}