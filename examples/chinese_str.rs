fn main() {
    let chinese = String::from("中文");
    println!(
        "length of 中文: {}, slice 1: {}",
        chinese.len(),
        &chinese[..3]
    );
}
