//! # あいうえお
//!
//! `かきくけこ` is a collection of utilities to make performing certain
//! calculations more convenient. さしすせそ
//!
//! #たちつてと
//!
//! `なにぬねの`は、ユーティリティの集まりであり、特定の計算をより便利に行うことができます。

/// Adds one to the number given.
/// 与えられた数値に1を足す。
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, ch14_01_comment_docs::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    if x > 2 { panic!("hoge"); } else { x + 1 }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
