trait Overview {
    fn overview(&self) -> String {
        String::from("Course")
    }
}

trait Another {
    fn hell (&self) {
        println!("welcome to hell");
    }
}

struct Course {
    headline: String,
    author: String,
}

impl Overview for Course {}
impl Another for Course {}

struct AnotherCourse {
    headline: String,
    author: String,
}

impl Overview for AnotherCourse {}

fn call_overview(item: & impl Overview) {
    println!("Overview {}", item.overview());
}

fn call_overview_generic<T: Overview>(item: &T) {
    println!("Overview {}", item.overview());
}

fn call_overview_t(item1: & impl Overview, item2: & impl Overview) {
    println!("Overview {}", item1.overview());
    println!("Overview {}", item2.overview());
}
fn call_overview_tt<T: Overview>(item1: &T, item2:&T) {
    println!("Overview {}", item1.overview());
    println!("Overview {}", item2.overview());
}

// 多绑定
fn call_mul_bind(item: & (impl Overview+Another)) {
    println!("Overview {}", item.overview());
    item.hell();
}

// fn call_mul_bind_generic<T: Overview + Another>(item: &T) {
//     println!("Overview {}", item.overview());
//     item.hell();
// }

fn call_mul_bind_generic<T>(item: &T)
where
    T: Overview+Another,
{
    println!("Overview {}", item.overview());
    item.hell();
}

fn main() {
    let c0 = Course {
        headline:"ff".to_owned(),
        author:"yy".to_owned(),
    };
    let c1 = Course {
        headline:"ff".to_owned(),
        author:"yy".to_owned(),
    };
    let c2 = AnotherCourse {
        headline:"ff".to_owned(),
        author:"yz".to_owned(),
    };
    call_overview(&c1);
    call_overview_generic(&c1);
    call_overview_t(&c1, &c2);
    println!("---------------------------");
    // call_overview_tt(&c1, &c2);// ^^^ expected `&Course`, found `&AnotherCourse`
    call_overview_tt(&c0, &c1);
    call_overview_t(&c0, &c1);

    call_mul_bind(&c1);
    call_mul_bind_generic(&c1);
}
