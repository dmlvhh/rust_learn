struct Person<'a> {
    name: &'a str,
    color: String,
    age: i32,
}

fn print(data: &str) {
    println!("{}", data);
}
fn print_string_borrow(data: &String) {
    println!("{}", data);
}

fn main() {
    let name = String::from("Value C++");
    let course = "Rust".to_owned();
    let new_name = name.replace("C++","CPP");
    println!("{name} {course} {new_name}");

    let rust = "\x52\x75\x73\x74";
    println!("{rust}");

    let color = "red".to_string();
    let name="anna";
    let people = Person {name, color, age: 42};

    //func
    let value = "value".to_owned();
    print(&value);
    print("data");
    // print_string_borrow("borrow");
    print_string_borrow(&value);
}
