fn main() {
    let s1 = String::from("Hello world");
    let s2 = first_word(&s1);
    println!("{}",s2);
    // s1.clear(); // コメントインするとエラー
}

// &str型は文字列のスライスを表現
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
