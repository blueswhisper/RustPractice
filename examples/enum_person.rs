// 在 struct 中合法的任意变量在 enum 同样是合法的
#[derive(Debug)]
enum Person {
    // 一个enum可以是unit-like struct
    Engineer,
    Scientist,
    // 或像一个元组解构体
    Height(i32),
    Weight(i32),
    // 或像一个普通结构体
    Info { name: String, height: i32 },
}

fn inspect(p: Person) {
    match p {
        Person::Engineer => println!("is Engineer"),
        Person::Scientist => println!("is Scientist"),
        Person::Height(i) => println!("has a height of {}", i),
        Person::Weight(i) => println!("has a weight of {}", i),
        Person::Info { name, height } => println!("{} is {} tall", name, height),
    }
}

fn main() {
    let person = Person::Height(18);
    let amira = Person::Weight(10);
    // `to_owned()` 从一个字符串 slice 创建一个具有所有权的 `String`。
    let dave = Person::Info {
        name: "Dave".to_owned(),
        height: 72,
    };
    let rebecca = Person::Scientist;
    let rohan = Person::Engineer;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(rohan);
}
