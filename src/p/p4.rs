use base64;
use std::fs;
use std::io::Read;
use std::str;

pub fn run(arg: &str) {
    println!("[p4]base64编码: {}", arg);

    let base64_123 = base64::encode("123");
    let base64_123_clone = base64_123.clone();

    println!("base64编码[123]: {}", base64_123);

    let decode_data = base64::decode(base64_123).unwrap();
    let decode_content = str::from_utf8(&decode_data).unwrap();

    println!("解码[{}]: {}", base64_123_clone, decode_content);

    // base64编码图片
    let mut image = fs::File::open("test.jpg").unwrap();
    let mut file_content = Vec::<u8>::new();

    image.read_to_end(&mut file_content).unwrap();

    fs::write("test.txt", base64::encode(file_content)).unwrap();
}
