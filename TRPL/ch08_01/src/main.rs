fn main() {
    // let s1 = String::from("hello");
    // let h = s1[0];
    let len = String::from("Здравствуйте").len();
    println!("{}", len);
    let hello = String::from("Здравствуйте");
    let s = &hello[0..4];
    println!("{}", s);
}
