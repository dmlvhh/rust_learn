fn main() {
    // 元组
    let tup = (500, "hi", 1.2);
    println!("tup elements {} {} {}",tup.0,tup.1,tup.2);

    let mut tup2 = (500, "hi", 1.2);
    println!("tup2 elements {} {} {}",tup2.0,tup2.1,tup2.2);
    tup2 = (501, "hidd", 1.24);
    tup2.1="hello";
    println!("tup2 elements {} {} {}",tup2.0,tup2.1,tup2.2);

    let tup3 = ();
    println!("tup3 {:?}",tup3);
    // println!("tup3 {}",tup3);

    // 数组
    let mut arr = [1,2,3,4,5,6,7,8,9,10];
    arr[0]=100;
    println!("arr len {}, first element is {}",arr.len(),arr[0]);

    for elem in arr {
        println!("elem {}",elem);
    }

    let ar = [2;3];
    for i in ar {
        println!("i {}",i);
    }

    // ownership
    let arr_item = [1,2,3];
    let tup_item = (2,"ff");
    println!("arr: {:?}",arr_item);
    println!("tup: {:?}",tup_item);
    let arr_ownership = arr_item;
    let tup_ownership = tup_item;
    println!("arr: {:?}",arr_item);
    println!("tup: {:?}",tup_item);

    let a = 3;
    let b = a;
    println!("{a}");

    let string_item = String::from("aaaa");
    let string_item_tt = string_item;
}
