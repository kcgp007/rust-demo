use std::ops::Add;
use tracing::info;

pub fn demo1() {
    let s = "ss";
    let s = s.to_owned() + "sss";
    info!(s);
}

pub fn demo2() {
    let n = 15;
    match n {
        n if n > 10 => info!("{} > 10", n),
        _ => info!("{} <= 10", n),
    };

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id @ 3..=7 } => info!("Found an id in range: {}", id),
        Message::Hello { id: 10..=12 } => info!("Found an id in another range"),
        Message::Hello { id } => info!("Found some other id: {}", id),
    };
}

pub fn demo3() {
    let a: i32 = 10;
    let b: u16 = 100;

    let b_ = b.try_into().unwrap();

    if a < b_ {
        info!("Ten is less than one hundred.");
    }
}

#[derive(Debug)]
struct Point<T: Add> {
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

pub fn demo4() {
    let p1 = Point {
        x: 1.1f32,
        y: 1.1f32,
    };
    let p2 = Point {
        x: 2.1f32,
        y: 2.1f32,
    };
    println!("{:?}", add(p1, p2));

    let p3 = Point { x: 1i32, y: 1i32 };
    let p4 = Point { x: 2i32, y: 2i32 };
    println!("{:?}", add(p3, p4));

    let i = 1;
    println!("{i}");
}

pub fn demo5() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn longest(x: &str, y: &str) -> String {
    if x.len() > y.len() {
        String::from(x)
    } else {
        String::from(y)
    }
}
