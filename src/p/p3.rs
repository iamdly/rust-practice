use std::fs;
use std::io::{BufReader, Read, Write};
use zip;

pub fn run(arg: &str) {
    println!("[p3]zip文件处理: {}", arg);

    // 创建test.zip文件
    println!("创建test.zip文件，并写入file1.txt和file2.txt两个文件");

    let file = fs::File::create("test.zip").unwrap();
    let mut zip_writer = zip::write::ZipWriter::new(file);

    // 写入文件到压缩包
    zip_writer
        .start_file("file1.txt", Default::default())
        .unwrap();
    zip_writer.write_all(b"Hello World File 1 Content").unwrap();

    zip_writer
        .start_file("file2.txt", Default::default())
        .unwrap();
    zip_writer.write_all(b"Hello File 2 Content").unwrap();

    zip_writer.finish().unwrap();

    // 读取test.zip文件
    let file = fs::File::open("./test.zip").unwrap();
    let reader = BufReader::new(file);
    let mut archive = zip::read::ZipArchive::new(reader).unwrap();

    // 遍历处理压缩包内文件
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let mut content = String::new();

        file.read_to_string(&mut content).unwrap();

        println!(
            "读取test.zip文件内文件 Index: {} File: [{}] Content: {}",
            i,
            file.name(),
            content
        );
    }
}
