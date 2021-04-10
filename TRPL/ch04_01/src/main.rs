// https://doc.rust-jp.rs/book-ja/ch04-01-what-is-ownership.html
// の図4-4がわかりやすい
//
// エラーが出る(所有権が移動しているから)
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}
