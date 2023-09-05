use std::time::Duration;
use std::{cell::RefCell, ops::Add, rc::Rc, thread};
use tracing::info;

pub fn _demo1() {
    let s = "ss";
    let s = s.to_owned() + "sss";
    info!(s);
}

pub fn _demo2() {
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

pub fn _demo3() {
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

fn _add<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

pub fn _demo4() {
    let p1 = Point {
        x: 1.1f32,
        y: 1.1f32,
    };
    let p2 = Point {
        x: 2.1f32,
        y: 2.1f32,
    };
    info!("{:?}", _add(p1, p2));

    let p3 = Point { x: 1i32, y: 1i32 };
    let p4 = Point { x: 2i32, y: 2i32 };
    info!("{:?}", _add(p3, p4));

    let i = 1;
    info!("{i}");
}

pub fn _demo5() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = _longest(string1.as_str(), string2.as_str());
        info!("The longest string is {}", result);
    }
}

fn _longest(x: &str, y: &str) -> String {
    if x.len() > y.len() {
        String::from(x)
    } else {
        String::from(y)
    }
}

#[derive(Debug)]
enum List {
    _Cons(i32, RefCell<Rc<List>>),
    _Nil,
}

impl List {
    fn _tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::_Cons(_, item) => Some(item),
            List::_Nil => None,
        }
    }
}

pub fn _demo6() {
    let a = Rc::new(List::_Cons(5, RefCell::new(Rc::new(List::_Nil))));

    info!("a的初始化rc计数 = {}", Rc::strong_count(&a));
    info!("a指向的节点 = {:?}", a._tail());

    // 创建`b`到`a`的引用
    let b = Rc::new(List::_Cons(10, RefCell::new(Rc::clone(&a))));

    info!("在b创建后，a的rc计数 = {}", Rc::strong_count(&a));
    info!("b的初始化rc计数 = {}", Rc::strong_count(&b));
    info!("b指向的节点 = {:?}", b._tail());

    // 利用RefCell的可变性，创建了`a`到`b`的引用
    if let Some(link) = a._tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    info!("在更改a后，b的rc计数 = {}", Rc::strong_count(&b));
    info!("在更改a后，a的rc计数 = {}", Rc::strong_count(&a));

    // 下面一行info!将导致循环引用
    // 我们可怜的8MB大小的main线程栈空间将被它冲垮，最终造成栈溢出
    info!("a next item = {:?}", a._tail());
}

pub fn demo7() {
    let v = vec![1, 2, 3, 4, 5];
    let v2 = v.clone();

    let handle = thread::spawn(|| {
        for i in v {
            info!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in v2 {
        info!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
