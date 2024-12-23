enum Flavor {
    Spicy,
    Sweet,
    Fruity,
}
struct Drink {
    flavor: Flavor,
    price: f64,
}

impl Drink {
    // 关联变量
    const MAX_PRICE: f64 = 10.0;
    // 方法
    fn buy(& self) {
        if self.price > Drink::MAX_PRICE {
            println!("I am poor");
            return;
        }
        println!("buy it");
    }

    // 关联函数
    fn new(price: f64) -> Self{
        Drink {
            flavor: Flavor::Fruity,
            price,
        }
    }
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Fruity => println!("Fuel"),
        Flavor::Spicy => println!("Spicy"),
        Flavor::Sweet => println!("Sweet"),
    }
    println!("{} ", drink.price);
}

fn main() {
    let sweet = Drink {
        flavor: Flavor::Sweet,
        price: 4.0,
    };
    println!("{} ", sweet.price);
    print_drink(sweet);

    let sweet = Drink::new(48.0);
    sweet.buy();
}
