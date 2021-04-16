// 整数のリストが与えられ、
// ベクタを使ってmean(平均値)、
// median(ソートされた時に真ん中に来る値)、
// mode(最も頻繁に出現する値; ハッシュマップがここでは有効活用できるでしょう)を返してください。

fn mean(values: &Vec<i32>) -> f64 {
    let mut sum = 0.0;
    for v in values {
        sum += f64::from(*v);
    }
    sum / values.len() as f64
}

fn median(values: &Vec<i32>) -> i32 {
    let mut copy = Vec::new();
    for v in values {
        copy.push(*v);
    }
    copy.sort();
    match copy.get(copy.len() / 2) {
        Some(&v) => v,
        None => -999,
    }
}

fn mode(values: &Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut map = HashMap::<i32, i32>::new();
    for &v in values {
        let count = map.entry(v).or_insert(0);
        *count += 1;
    }
    let mut mode = 0;
    let mut max_count = 0;
    for (key, v) in map {
        if max_count < v {
            max_count = v;
            mode = key
        }
    }
    mode
}

fn main() {
    let values = vec![5, 2, 7, 1, 3, 2, 9];
    println!("{}", mean(&values));
    println!("{}", median(&values));
    println!("{}", mode(&values));
}
