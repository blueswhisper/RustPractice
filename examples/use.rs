// 将`deeply::nested::function` 路径绑定到 `other_function`
use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    other_function();

    println!("Entering block");
    {
        // 这和 `use deeply::nested::function as function` 等价。
        // 此 `function()` 将覆盖掉的外部同名函数
        use deeply::nested::function;
        function();

        //  `use` 绑定拥有局部作用域。在这个例子中，`function()`
        //  的覆盖只限定在这个代码块中。
        println!("Leaving block");
    }
    function();
}
