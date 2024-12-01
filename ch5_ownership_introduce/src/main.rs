fn get_length(s: String) -> usize {
    println!("String: {}", s);
    // 函数结束之后 main::s1也销毁了
    s.len()
}
fn main() {
    // copy
    let c1 = 1;
    let c2 = c1;
    println!("{}",c1);
    let s1 = String::from("hello");
    let s2 = s1.clone(); // s1 的所有权转移给s2

    println!("{}",s1);

    let len = get_length(s1);
    println!("{}",len);

    let back = first_word("hello world");
    println!("{}",back);
    let back = first_word("we are the world");
    println!("{}",back);
}


fn dangle() ->String {
    "hello".to_owned()
}

// 静态
fn dangle_static()-> &'static str {
    "hahahhaha"
}
// String 与 &str vec u8 ref
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
        return &s[0..i];
        }
    }
    &s[..]
}