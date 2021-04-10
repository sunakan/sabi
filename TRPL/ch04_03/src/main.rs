fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // ここで、sはスコープ外になる。
  // けど、参照しているものの所有権は持っていないので
  // 何も起きない(dropも起きない)
  // この引数に参照をとることを "借用" という
