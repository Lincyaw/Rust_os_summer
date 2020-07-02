use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::stdin;

fn main() {
    // 创建指向所需的文件的路径
    let mut file = String::new();
    println!("What's your path?");
    stdin().read_line(&mut file).unwrap();

    let path = Path::new(&file);
    let display = path.display();


    // // 创建指向所需的文件的路径
    // let path = Path::new("hello.txt");
    // let display = path.display();

    // 以只读方式打开路径，返回 `io::Result<File>`
    let mut file = match File::open(&path) {
        // `io::Error` 的 `description` 方法返回一个描述错误的字符串。
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.to_string()),
        Ok(file) => file,
    };

    // 读取文件内容到一个字符串，返回 `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                           why.to_string()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // `file` 离开作用域，并且 `hello.txt` 文件将被关闭。
}

