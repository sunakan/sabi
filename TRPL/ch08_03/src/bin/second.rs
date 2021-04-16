// 文字列をピッグ・ラテン(訳注: 英語の言葉遊びの一つ)に変換してください。
// 各単語の最初の子音は、 単語の終端に移り、"ay"が足されます。
// 従って、"first"は"irst-fay"になります。
// ただし、 母音で始まる単語には、お尻に"hay"が付け足されます
// ("apple"は"apple-hay"になります)。
// UTF-8エンコードに関する詳細を心に留めておいてください！
fn pig_latin(s: &str) -> String {
    match s.chars().nth(0) {
        Some(v) => v.to_string(),
        None => String::from("hello"),
    }
}

fn main() {
    let v = "apple";
    println!("{}", pig_latin(&v));
}
