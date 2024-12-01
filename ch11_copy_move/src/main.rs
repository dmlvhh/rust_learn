#[derive(Debug,Copy,Clone)]
struct Book {
    page: i32,
    rating: f64,
}

fn main() {
    let x = vec![1,2,3,4];
    let y = x.clone();
    println!("{:?}", y);
    println!("{:?}", x);

    let x = "rust".to_string();
    let y  = x.clone();
    println!("{:?}", x);

    let b1 = Book { page: 3, rating: 1.0 };
    let b2 = b1; //move
    println!("{:?}", b2);
}
