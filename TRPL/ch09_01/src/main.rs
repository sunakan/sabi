use std::fs::File;
use std::io::ErrorKind;

// RUST_BACKTRACE=full cargo run
// pani()をcallするとpanic
fn pani() {
    let v = vec![1, 2, 3];
    v[99];
}
fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) if error.kind() == ErrorKind::NotFound => {
            println!("====");
            println!("{:?}", error);
            println!("====");
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("ファイル作成をしようとしましたが、失敗 {:?}", e)
                }
            }
        },
        Err(error) => {
            panic!("{:?}", error)
        }
    };
}
