enum Color {
    Red,
    Yellow,
    Blue,
    Green,
}

fn print_color(my_color: Color) {
    match my_color {
        Color::Red => println!("red"),
        Color::Yellow => println!("yellow"),
        Color::Blue => println!("blue"),
        Color::Green => println!("green"),
    }
}
enum BuildingLocation {
    Number(i32),
    Name(String),
    Unknown,
}

impl BuildingLocation {
    fn print_location(&self) {
        match self {
            BuildingLocation::Number(c)=> println!("{} building number", c),
            BuildingLocation::Name(s)=> println!("{} building name", *s),
            BuildingLocation::Unknown => println!("building unknown"),
        }
    }
}

fn main() {
    let a  = Color::Red;
    print_color(a);
    // let b = a;

    let house = BuildingLocation::Name("House".to_string());
    let house = BuildingLocation::Number(1);
    let house = BuildingLocation::Unknown;
    house.print_location();
}
