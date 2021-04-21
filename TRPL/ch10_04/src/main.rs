fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("a");
    let mut result = &string1;
    {
        //let string2 = "xyz";
        let string2 = String::from("xyz");
        //let result = longest(string1.as_str(), string2.as_str());
        // コンパイルエラーはわかる、、、がエラー内容が謎
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    //// 最長の文字列は、{}です
    println!("The longest string is {}", result);
}
