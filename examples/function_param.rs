fn call_me<F: Fn()>(f: F) {
    f()
}

fn function() {
    println!("I am a function");
}

fn main() {
    let closure = || println!("I'm a closure");
    call_me(function);
    call_me(closure);
}
