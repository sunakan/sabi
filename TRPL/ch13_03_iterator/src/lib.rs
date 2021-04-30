#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    let r1: Vec<Shoe> = shoes.into_iter().filter(|s| s.size == shoe_size).collect();
    // 入れたらコンパイル失敗（into_iterによってmove）
    //let r2 = shoes.into_iter()
    //    .filter(|s| s.size == shoe_size)
    //    .collect();
    r1
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}

#[cfg(test)]
mod test {
    #[test]
    fn iterator_sum() {
        let v1 = [1, 2, 3];
        let v1_iter = v1.iter();
        let total: u32 = v1_iter.sum();
        assert_eq!(total, 6);
    }

    // 2回iter()メソッドをcallした時、どうなっているか？
    #[test]
    fn second_time_iterator_sum() {
        let v1 = [1, 2, 3];
        let v1_iter = v1.iter();
        let total: u32 = v1_iter.sum();
        assert_eq!(total, 6);

        let v1_iter2 = v1.iter();
        let total: u32 = v1_iter2.sum();
        assert_eq!(total, 6);
    }

    // イテレータアダプタmapを使ってみる
    #[test]
    fn use_map() {
        let v1 = [1, 2, 3];
        let v1_iter = v1.iter();
        let iter2 = v1_iter.map(|x| 2 * x);
        let total: u32 = iter2.sum();
        assert_eq!(total, 12);
    }
}
