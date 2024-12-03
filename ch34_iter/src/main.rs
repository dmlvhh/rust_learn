fn main() {
    let vec = vec![1, 2, 3,4,5,6,7,8,9];
    //iter()
    for &item in vec.iter() {
        println!("{}",item);
    }
    println!("{:?}",vec);

    // 可变引用
    let mut vec = vec![1,2,3,4,5,6,7,8,9];
    for item in vec.iter_mut() {
        *item *= 2;
    }
    println!("{:?}",vec);

    //所有权转移
    let vec = vec![1,2,3,4,5,6,7,8,9];
    for item in vec.into_iter() {
        println!("{}", item);
    }
    // println!("{:?}",vec);
}
