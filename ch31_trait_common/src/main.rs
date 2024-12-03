#[derive(Debug,Clone)]
enum Race {
    White,
    Yellow,
    Black,
}
impl PartialEq for Race {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Race::White, Race::White) => true,
            (Race::Yellow, Race::Yellow) => true,
            (Race::Black, Race::Black) => true,
            _ => false,
        }
    }
}

#[derive(Debug,Clone)]
struct User {
    id: u32,
    name: String,
    race: Race,
}

impl PartialEq for User {
    fn eq(&self, other: &User) -> bool {
        self.id == other.id && self.name == other.name && self.race == other.race
    }
}

fn main() {
    let user = User {
        id: 3,
        name: "John".to_owned(),
        race: Race::Yellow,
    };
    println!("{:#?}", user);
    let user2 = user.clone();
    // let user2 = user;
    println!("{:#?}", user2);
    // println!("{:#?}", user);
    println!("{}", user == user2);
}
