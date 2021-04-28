

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
