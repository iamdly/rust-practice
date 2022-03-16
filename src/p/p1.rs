use std::{fs, io};

fn has_exists(path: &str) -> bool {
    match fs::read(path) {
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => false,
            _ => true,
        },
        _ => true,
    }
}

pub fn run(arg: String) {
    println!("[p1]文件读写: {}", arg);

    // 写入文件内容
    fs::write("test.txt", "Hello World~").unwrap();

    // 读取文件内容
    let content = fs::read_to_string("test.txt").unwrap();

    println!("文件test.txt内容：{}", content);

    // 修改文件名称
    fs::rename("test.txt", "test_rename.txt").unwrap();

    // 判断文件是否存在
    println!("test.txt文件是否存在：{}", has_exists("test.txt"));
    println!(
        "test_rename.txt文件是否存在：{}",
        has_exists("test_rename.txt")
    );

    // 删除文件
    fs::remove_file("test_rename.txt").unwrap();
}
