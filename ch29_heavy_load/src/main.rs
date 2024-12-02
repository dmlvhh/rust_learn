use std::ops::Add;
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// T的这样类型 它可以执行相加的操作
impl <T> Add for Point<T>
where
    T: Add<Output=T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
fn main() {
    let c1 = Point { x: 1, y: 2 };
    let c2 = Point { x: 3, y: 4 };
    println!("{:?}", c1 + c2);

    let c1 = Point { x: 1.1, y: 2.8 };
    let c2 = Point { x: 3.0, y: 4.4 };
    println!("{:?}", c1 + c2);
}
