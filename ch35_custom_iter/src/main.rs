#[derive(Debug)]
struct Stack<T> {
    item: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { item: Vec::new() }
    }
    // 入栈
    fn push(&mut self, item: T) {
        self.item.push(item);
    }
    // 出栈
    fn pop(&mut self) -> Option<T> {
        self.item.pop()
    }
    // 不可变引用
    fn iter(&self) -> std::slice::Iter<T> {
        self.item.iter()
    }
    fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.item.iter_mut()
    }
    fn into_iter(self) -> std::vec::IntoIter<T> {
        self.item.into_iter()
    }
}
fn main() {
    let mut my_stack = Stack::new();
    my_stack.push(1);
    my_stack.push(2);
    my_stack.push(3);
    for item in my_stack.iter() {
        println!("item {}", item);
    }
    println!("{:?}", my_stack);

    for item in my_stack.iter_mut() {
        *item *= 2;
    }
    println!("{:?}", my_stack);
    for item in my_stack.into_iter() {
        println!("{}", item);
    }
    // println!("{:?}", my_stack);
}
