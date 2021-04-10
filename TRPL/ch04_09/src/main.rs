fn main() {
    //-----------------------------------
    let s1 = "Hello world";
    let s2 = first_word(s1);
    println!("{}", s2);
    //-----------------------------------
    let s3 = String::from("Hello2 world");
    let s4 = first_word(&s3[..]);
    println!("{}", s4);
}

// &str型は文字列のスライスを表現
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
