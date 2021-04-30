struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping with data `{}`!", self.data);
    }
}

fn main() {
    println!("Hello, world!");
    let c = CustomSmartPointer {data: String::from("aaaaaaaaaaaaaa")};
    let d = CustomSmartPointer {data: String::from("bbbbbbbbbbbbbb")};
    println!("Hello, world!");
}
