fn main() {
    fizzbuzz_to(100);
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // 极端情况下,提前返回false
    if rhs == 0 {
        return false;
    }
    //返回值语句不要写;
    lhs % rhs == 0
}

fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// 当函数返回 `()` 时，可以从标记中删除返回类型
fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n)
    }
}
