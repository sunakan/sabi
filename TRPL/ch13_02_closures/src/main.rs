use std::ptr::eq;

fn main() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
    // これは通らない
    // fn equal_to_x(z: i32) -> bool { z == x }
    assert!(equal_to_x(y));
}
