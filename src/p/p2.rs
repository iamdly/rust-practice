use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct UserData {
    name: String,
    age: u8,
}

pub fn run(arg: &str) {
    println!("[p2]JSON数据处理: {}", arg);

    let user = UserData {
        name: "张三".to_string(),
        age: 18,
    };

    println!("UserData: {:?}", user);

    println!(
        "UserData To JSON: {}",
        serde_json::to_string(&user).unwrap()
    );

    let meta_user_data = r#"
        {
            "name": "李四",
            "age": 22
        }
    "#;

    println!("MetaUserData: {}", meta_user_data);

    let to_user: UserData = serde_json::from_str(meta_user_data).unwrap();

    println!("MetaUserData To Struct: {:?}", to_user);
}
