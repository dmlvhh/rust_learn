fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }else {
        y
    }
}
fn longest_str<'a,'b,'out>(x: &'a str, y: &'b str) -> &'out str
where
    'a: 'out,
    'b: 'out,
{
    if x.len() > y.len() {
        x
    }else {
        y
    }
}
fn no_need(s: &'static str,s1: &str) -> &'static str {
    s
}
fn main() {
    println!("{}", no_need("hello",""));

    let x = "hello world";
    let y = "hello";
    println!("longest {}", longest(x, y));

    let result: &str;
    {
        let r2 = "world";
        result = longest_str(x,r2);
        println!("The longest string is : {}", result);
    }
}
