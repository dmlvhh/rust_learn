#[derive(Debug)]

struct User {
    name: String,
    score: u64,
}

// fn sort_score(users: &mut Vec<User>) {
//     users.sort_by_key(sort_helper);
// }
//
// fn sort_helper(u: &User)->u64 {
//     u.score
// }

fn sort_score_closure(users: &mut Vec<User>) {
    users.sort_by_key(|u| u.score);
}

fn main() {
    let f = |a,b| a + b;
    println!("{:?}", f(1.0,2.9));
    let a = User {
        name: "Alice".to_owned(),
        score: 42,
    };
    let b = User {
        name: "Bob".to_owned(),
        score: 66,
    };
    let c = User {
        name: "Caul".to_owned(),
        score: 33,
    };
    let d = User {
        name: "David".to_owned(),
        score: 88,
    };
    let mut users = vec![a, b, c, d];
    // sort_score(&mut users);
    sort_score_closure(&mut users);
    println!("{:?}", users);
}
