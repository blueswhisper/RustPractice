use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 对 `self` 解引用，并通过解构创建一个指向 `vec` 的引用。
        let List(ref vec) = *self;
        try!(write!(f, "["));

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                try!(write!(f, ", "));
            }
            try!(write!(f, "{}: {}", count, v));
        }
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3, 4]);
    println!("{}", v);
}
