struct MyBox<T>(T);
struct MyBox2<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> MyBox2<T> {
    fn new(x: T) -> MyBox2<T> {
        MyBox2(x)
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox2<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    a();
    b();
    c();
}

fn a() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, y.0);
    // Derefが実装されてないから
    //assert_eq!(5, *y);// これはコンパイルできない
}

fn b() {
    let x = 5;
    let y = MyBox2::new(x);
    assert_eq!(5, x);
    assert_eq!(5, y.0);
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref()));
}

fn c() {
    let m = MyBox2::new(String::from("Rust"));
    let n = &m;
    // &MyBox2<String>を渡しているのに、hello側では&strとして受けている
    hello(n);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
