struct Obj {}

trait Overview {
    fn overview(&self) -> String {
        String::from("overview")
    }
}

impl Overview for Obj {
    fn overview(&self) -> String {
        String::from("Obj")
    }
}
// 不可变引用
fn call_obj(item: & impl Overview){
    println!("Overview {}", item.overview());
}
// Move
fn call_obj_box(item: Box<dyn  Overview>){
    println!("Overview {}", item.overview());
}

trait Sale {
    fn amount(&self) -> f64;
}

struct Common(f64);

impl Sale for Common {
    fn amount(&self) -> f64 {
        self.0
    }
}

struct TenDiscount(f64);
impl Sale for TenDiscount {
    fn amount(&self) -> f64 {
        self.0 - 10.0
    }
}

struct TenPercentDiscount(f64);
impl Sale for TenPercentDiscount {
    fn amount(&self) -> f64 {
        self.0 * 0.9
    }
}

fn calculate(sales: &Vec<Box<dyn Sale>>) ->f64 {
    sales.iter().map(|s| s.amount()).sum()
}

fn main() {
    let a  = Obj{};
    call_obj(&a);
    println!("{}", a.overview());
    let b_a = Box::new(Obj{});
    call_obj_box(b_a);
    // println!("{}", b_a.overview());

    let c  = Box::new(Common(100.0));
    let t1  = Box::new(TenDiscount(100.0));
    let t2  = Box::new(TenPercentDiscount(100.0));

    let sales:Vec<Box<dyn Sale>> = vec![c, t1, t2];
    println!("pay {:?}", calculate(&sales));
}
