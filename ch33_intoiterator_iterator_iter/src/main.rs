fn main() {
    //vec
    let v = vec![1,2,3,4,5];
    // 转换为迭代器
    let iter = v.into_iter();//转化为迭代器，所有权转移
    let sum = iter.sum::<i32>();
    println!("{:?}", sum);
    //array
    let array = [1,2,3,4,5];
    let iter:std::slice::Iter<'_,i32> = array.iter();
    let sum = iter.sum::<i32>();
    println!("{:?}", sum);
    // array.iter().for_each(|x| println!("{:?}", x));


    let text = "hello world wonderful world";
    let iter = text.chars();
    let uppercase = iter.map(|c| c.to_ascii_uppercase()).collect::<String>();
    println!("{:?}", uppercase);
    println!("{:?}",text)

    //chars
}
