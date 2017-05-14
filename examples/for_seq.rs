// 创建计算器的一个最简便的方法就是使用区间标记 a..b。
// 这 会生成从 a（包含此值） 到 b （不含此值）增幅为 1 的一系列值。
fn main() {
    for n in 1..101 {
    	if n % 15 == 0 {
    	    println!("fizzbuzz");
    	} else if n % 3 == 0 {
    	    println!("fizz");
    	} else if n % 5 == 0 {
    		println!("buzz");
    	} else {
    	    println!("{}", n);
    	}
    }
}