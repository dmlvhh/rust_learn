fn main() {
    // loop {
    //     println!("Ctrl+C");
    //     std::thread::sleep(std::time::Duration::from_secs(1));
    // }

    let mut i = 0;
    while i < 10 {
        print!("{} ", i);
        i += 1;
    }
    println!();
    println!("for circulate");
    let arr = [1,2,3,4,5,6,7];
    for element in arr {
        print!("{} ", element);
    }
    println!();
    for i in 0..10 {
        print!("{} ", i);
    }
    println!();
    for i in 0..=10 {
        print!("{} ", i);
    }
    println!();
    // break
    let arr = [1,2,3,4,5,6,7,8,9,10,11,12];
    for element in arr {
        if element == 10 {
            break
        }
        print!("{} ", element);
    }
    println!();
    // continue
    let arr = [1,2,3,4,5,6,7,8,9,10,11,12];
    for element in arr {
        if element == 10 {
            continue
        }
        print!("{} ", element);
    }

    println!();
     // loop {
    'outer: loop {
        println!("outer");
        loop {
            println!("inner");
            // break;
            break 'outer;
        }
    }

    println!();

    // 循环写法
    let numbers = [1,2,3,4,5,6,7,8,9,10,11,12];
    let mut for_numbers = Vec::new();
    for &number in numbers.iter() {
        let item = number * number;
        for_numbers.push(item);
    }
    println!("for : {:?}", for_numbers);

    // 迭代写法
    let numbers = [1,2,3,4,5,6,7,8,9,10,11,12].to_vec();
    let mut for_number: Vec<_> =numbers.iter().map(|&x| x*x).collect();
    println!("iter: {:?}", for_numbers);
}
