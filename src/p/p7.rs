use pdf::file::File;
use pdf::object::Resolve;
use pdf::object::*;
use std::fs;

pub fn run() {
    println!("[p7]读取PDF内图片文件");

    let file = File::open("Y:/test.pdf").unwrap();
    let mut index = 0;

    for page in file.pages() {
        let page = page.unwrap();
        let resources = page.resources().unwrap();

        for (_, r) in resources.xobjects.iter() {
            let img = file.get(*r).unwrap();
            let img = match *img {
                XObject::Image(ref im) => im,
                _ => continue,
            };

            if let Some(jpeg_data) = img.as_jpeg() {
                if let Err(err) = fs::write(format!("Y:/{}.jpg", index), jpeg_data) {
                    println!("写入文件失败：{:?}", err);
                } else {
                    index += 1;
                }
            }
        }
    }
}
