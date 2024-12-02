fn add(x:i32,y:i32)->i32{
    x + y
}

fn change_i32(mut x:i32){
    x = x + 4;
    println!("fn {x}");
}

fn modify_i32(x: &mut i32){
    *x += 4;
}
#[derive(Copy, Clone)]
struct Point{
    x:i32,
    y:i32,
}
fn print_point(point: Point){
    println!("Point {:?} {:?}", point.x, point.y);
}

fn main() {
    let a = 1;
    let b = 2;
    let c: i32 = add(a, b);
    println!("c: {}",c);

    let mut x = 1;
    change_i32(x);
    println!("{}", x);

    modify_i32(&mut x);
    println!("{}", x);

    let s = Point{x:1,y:2};
    print_point(s);
    println!("{}", s.x);
}
