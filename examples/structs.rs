// 单元结构体,没有字段,主要用于泛型场景（现在还不知道怎么用?）
struct Nil;

// 元组结构体,字段没有名字,构造类似元组
struct Pair(i32, f32);

// 带有两个有名字段的结构体,类似c语言的结构体
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
	p1: Point,
	p2: Point,
}

fn rect_area(rectangle: &Rectangle) -> f32 {
    let Rectangle {
    	p1: Point {x: x1, y: y1},
    	p2: Point {x: x2, y: y2},
    } = *rectangle;
    ((x1 - x2) * (y1 - y2)).abs()
}

fn square(point: &Point, edge: f32) -> (Point, f32, f32) {
	let Point{x: x1, y: y1} = *point;
	(Point { x: (x1 - edge), y: (y1 - edge) }, edge, edge)
}

fn main() {
    let point: Point = Point { x: 0.3, y: 0.4 };
    println!("point coordiantes: ({}, {})", point.x, point.y);

    // 解构(decomposition)point
    let Point { x: my_x, y: my_y } = point;

    let _ractangle = Rectangle {
    	p1: Point {
    		x: my_y, y: my_x
    	},
    	p2: point,
    };

    let _nil = Nil;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构元组结构体
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("rectangle {:?} area: {}", _ractangle, rect_area(&_ractangle));

    let point1: Point = Point { x: 0.1, y: 0.2 };
    let rect_info = square(&point1, 3.0f32);
    println!("rectangle build from point: {:?} with edge: {}, left down point: {:?}, width: {}, heigth: {}", point1, 3.0f32, rect_info.0, rect_info.1, rect_info.2);
}