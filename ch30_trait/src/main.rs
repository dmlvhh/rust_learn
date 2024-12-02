use std::collections::VecDeque;
// 多态
trait Driver {
    fn drive(&self);
}

struct Car;

impl Driver for Car {
    fn drive(&self) {
        println!("Car");
    }
}

struct Suv;

impl Driver for Suv {
    fn drive(&self) {
        println!("Suv");
    }
}

fn road(vehicle: &dyn Driver) {
    vehicle.drive();
}

// 继承思想
// 单向特质
trait Queue {
    fn len(&self) -> usize;
    fn push_back(&mut self, x: i32);
    fn pop_front(&mut self) -> Option<i32>;
}
// 双向特质
trait Deque: Queue {
    fn push_front(&mut self, x: i32);
    fn pop_back(&mut self) -> Option<i32>;
}
#[derive(Debug)]
struct List {
    data:VecDeque<i32>,
}

impl List {
    fn new() -> Self {
        let data = VecDeque::<i32>::new();
        Self { data }
    }
}

impl Deque for List {
    fn push_front(&mut self, x: i32) {
        self.data.push_front(x);
    }
    fn pop_back(&mut self) -> Option<i32> {
        self.data.pop_back()
    }
}

impl Queue for List {
    fn len(&self) -> usize {
        self.data.len()
    }
    fn push_back(&mut self, x: i32) {
        self.data.push_back(x);
    }
    fn pop_front(&mut self) -> Option<i32> {
        self.data.pop_front()
    }
}

fn main() {
    // road(&Car);
    // road(&Suv);

    let mut l = List::new();
    l.push_back(1);
    l.push_front(1);
    println!("{:?}", l);
    l.push_front(2);
    println!("{:?}", l);
    l.push_back(2);
    println!("{:?}", l);
    println!("{}",l.pop_back().unwrap());
    println!("{:?}", l);
}
