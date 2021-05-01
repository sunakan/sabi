//==============================================================================
// $ cargo test -- --nocapture
//==============================================================================
#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io;
    use tempfile::NamedTempFile;

    #[test]
    fn tempfile() {
        println!("=====[tempfile]");
        let tf: io::Result<File> = tempfile::tempfile();
        match tf {
            Ok(f) => {
                println!("===");
                println!("{:?}", f.metadata());
                println!("===");
            },
            Err(e) => panic!("panicked!!! {}",e)
        }
    }

    #[test]
    fn named_tempfile() {
        println!("=====[named_tempfile]");
        let tf: io::Result<NamedTempFile> = NamedTempFile::new();
        match tf {
            Ok(f) => {
                println!("===");
                println!("{:?}", f.as_file().metadata());
                println!("===");
            },
            Err(e) => panic!("panicked!!! {}",e)
        }
    }

    #[test]
    fn named_tempfile__into_parts() {
        println!("=====[named_tempfile__into_parts]");
        let tf: io::Result<NamedTempFile> = NamedTempFile::new();
        match tf {
            Ok(f) => {
                let a = f.into_parts();
                println!("===");
                println!("{:?}", a);
                println!("===");
            },
            Err(e) => panic!("panicked!!! {}",e)
        }
    }
}
