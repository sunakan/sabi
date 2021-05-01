use std::path::Path;
use std::fs::File;
use std::error::Error;

fn main() {
    let path = Path::new("hello.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        // `io::Error`の`description`メソッドはエラーを説明する文字列を返す。
        Err(reason) => panic!("couldn't open {}: {}", display,
                           reason.description()),
        Ok(file) => file,
    };

}
