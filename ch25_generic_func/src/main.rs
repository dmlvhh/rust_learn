fn swap<T>(a:T, b:T) -> (T, T) {
    (b,a)
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x:T, y:T) -> Self {
        Point { x, y }
    }
    fn get_coordinates(&self) -> (&T, &T) {
        (&self.x,&self.y)
    }
}
fn main() {
    let result = swap::<f64>(1.0, 2.0);
    let result = swap(1.0, 2.0);
    println!("{:?}", result);
    let str = swap('a', 'b');
    println!("str.0 {:?}, str.1 {:?}", str.0, str.1);
    let str = swap(str.0, str.1);
    println!("str.0 {:?}, str.1 {:?}", str.0, str.1);

    let i32_point = Point::new(2, 3);
    let i64_point = Point::new(2.0, 3.0);

    let (x1, y1) = i32_point.get_coordinates();
    let (x2, y2) = i64_point.get_coordinates();
    println!("i32_point {:?}, i64_point {:?}", x1, x2);
    println!("f64_point {:?}, f64.point {:?}", x1, x2);
    let string_point = Point::new("xxx".to_owned(), "yyy".to_owned());
    println!("string point: x = {}, y = {}", string_point.x, string_point.x);
}
