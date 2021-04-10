fn main() {
    let s = String::from("hello");
    change(&s);
}

// これはエラーがでる
// 理由：借用に対しては変更できない
fn change(some_string: &String) {
    some_string.push_str(", world");
}
