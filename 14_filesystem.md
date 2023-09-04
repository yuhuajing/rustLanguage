# Filesystem

`std::fs::File`包读取文件，`File::create`创建或加载文件， `File::open`打开文件
> https://doc.rust-lang.org/std/fs/struct.File.html

```text
use std::fs::File;
use std::io::prelude::*;
use std::{fs, io};

fn main() -> std::io::Result<()> {
    // let mut file = File::create("foo.txt")?;
    // file.write_all(b"Hello, world!")?;
    // Ok(())

    // let mut file = File::open("foo.txt")?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;
    // println!("{}",contents);
    // assert_eq!(contents, "Hello, world!");
    // Ok(())

    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let prefix = match file_type {
            t if t.is_dir() => "D",
            t if t.is_file() => "F",
            t if t.is_symlink() => "L",
            _ => "?",
        };
        println!("{prefix} {}", entry.path().display());
    }

    Ok(())
}
```