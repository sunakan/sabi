fn main() {
    let reference_to_nothing = dangel();
}

// 宙に浮いた参照
// これはエラー
// 所有権がそのままだけど、参照だけもってかれたら無理ゲー
// （ライフタイムが関連しているらしい）
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
