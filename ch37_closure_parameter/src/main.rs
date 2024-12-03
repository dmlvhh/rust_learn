fn main() {
    //Fn 不可变引用获取外部参数
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let fn_func = |s| {
        println!("{}", s1);
        println!("i am {}", s);
        println!("{}", s1);
    };
    fn_func("yz".to_owned());
    fn_func("原子".to_owned());
    println!("{s1} {s2}");

    //FnMut 可变引用获取外部参数
    let mut s1 = String::from("hello");
    let mut s2 = String::from("world");
    let mut fn_func = |s| {
        s1.push_str("😀");
        s2.push_str("😀");
        println!("{}", s1);
        println!("i am {}", s);
        println!("{}", s1);
    };
    fn_func("yz".to_owned());
    fn_func("原子".to_owned());
    println!("{s1} {s2}");

    //所欲权转移，由编译器根据我们的代码来判读
    let s1 = String::from("111111");
    let fn_once_func = || {
        println!("{s1}");
        std::mem::drop(s1);
    };
    fn_once_func();
    // println!("{s1}");

    let s1 = String::from("222222");
    let move_fn = move || {
        println!("{s1}");
    };
    move_fn();
    // println!("{s1}");
    let s1 = String::from("333333");
    std::thread::spawn(move || println!("{s1}"));
}
