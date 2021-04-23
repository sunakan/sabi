fn join<'a, 'b>(a: &str, b: &str) -> String {
    format!("{}{}", a.to_string(), b.to_string())
}

fn main() {
    let a = "a".to_string();
    let b = "b".to_string();
    println!("{}", join(&a, &b))
    //println!("{}", format!("{}{}", a, b));
}
