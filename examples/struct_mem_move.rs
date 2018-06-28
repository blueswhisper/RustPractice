struct A {
    name: String,
}

fn main() {
    let a: A = A {
        name: String::from("a"),
    };
    let b = a.name;
    // comment next to sedd compile error
    //println!("{}, {}", a.name, b);
}
