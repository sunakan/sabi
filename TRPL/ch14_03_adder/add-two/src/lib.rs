pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_bbbb() {
        assert_eq!(5, add_two(3));
    }
}
