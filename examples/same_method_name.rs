struct A ;

trait One {
    fn act(&self);
}

trait Two {
    fn act(&self);
}

impl One for A {
    fn act(&self) {
        println!("call One");
    }
}

impl Two for A {
    fn act(&self) {
        println!("call Two");
    }
}

fn main() {
    let a = A;
    Two::act(&a);
}
