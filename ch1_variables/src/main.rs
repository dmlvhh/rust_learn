fn main() {
    // 不可变与命名
    let _nice_count: i32 = 100;
    let _nice_count = 100;
    // nice_count = 23;

    //声明变量
    let mut _count = 3;
    _count = 4;

    //Shadowing
    let x = 5;
    {
        //命名空间
        let x = 10;
        println!("inner x : {}", x);
    } // 内部的x被销毁了
    println!("Quter x : {x}");
    let x = "hello"; // 在同一作用域下重新声明了x,最终覆盖了之前的x
    print!("New x : {x}");
    // x = "hello";

    //mut 关键字表示这个变量是可变的
    let mut x = "this";
    println!("x:{x}");
    x = "that";
    println!("Mut x：{x}");
}
