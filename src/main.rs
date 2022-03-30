mod p;

use crate::p::{p1, p2, p3, p4, p5, p6, p7, p8, p9};
use clap::{ArgEnum, Parser};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// 练习案例 p1:文件读写 p2:JSON数据处理 p3:zip文件处理 p4:base64编码
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
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
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
        P::P3 => {
            p3::run(args.sub_p.as_str());
        }
        P::P4 => {
            p4::run(args.sub_p.as_str());
        }
        P::P5 => {
            p5::run(args.sub_p.as_str());
        }
        P::P6 => {
            p6::run(args.sub_p.as_str());
        }
        P::P7 => {
            p7::run();
        }
        P::P8 => {
            p8::run();
        }
        P::P9 => {
            p9::run();
        }
    }
}
