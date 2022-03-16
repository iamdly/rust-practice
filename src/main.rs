mod p;

use crate::p::p1;
use crate::p::p2;
use clap::{ArgEnum, Parser};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// 练习案例 p1:文件读写 p2:JSON数据处理
    #[clap(long, short, arg_enum)]
    p: P,

    #[clap(long, short, default_value_t = String::from(""))]
    sub_p: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
enum P {
    P1,
    P2,
    P3,
}

fn main() {
    let args: Args = Args::parse();

    match args.p {
        P::P1 => {
            p1::run(args.sub_p);
        }
        P::P2 => {
            p2::run(args.sub_p.as_str());
        }
        _ => {
            println!("未知");
        }
    }
}