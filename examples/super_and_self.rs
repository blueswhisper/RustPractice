fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::funtion()`");
        }
    }

    pub fn indirect_call() {
        //从这个作用域中访问所有名为`function`的函数
        print!("called, `my::indirect_call()`, that\n> ");

        // `self`关键字表示当前模块的作用域--在这个例子中是`my`
        // 调用`self::function()`和直接访问`funtion()`两者都得到相同的结果
        // 因为它们表示相同的函数
        self::function();
        function();

        //我们也可以使用`self`来访问`my`的另一个模块
        self::cool::function();

        // `super`关键字表示父级作用域(在`my`模块的外面)
        super::function();

        // 在将在*crate*作用域内绑定`cool::funtion`
        // 在这个例子中,crate作用域是最外面的作用域
        {
            use cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call()
}
