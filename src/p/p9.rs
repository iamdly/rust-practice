use time::ext::*;
use time::{format_description, macros::time, OffsetDateTime};

pub fn run() {
    println!("[p8]日期时间处理");

    let now_utc = OffsetDateTime::now_utc();
    let now_local = OffsetDateTime::now_local().unwrap();

    println!(
        "UTC: {}, {}, {}, {}, {}, {}",
        now_utc.year(),
        now_utc.month(),
        now_utc.day(),
        now_utc.hour(),
        now_utc.minute(),
        now_utc.second()
    );

    println!(
        "LOCAL: {}, {}, {}, {}, {}, {}",
        now_local.year(),
        now_local.month(),
        now_local.day(),
        now_local.hour(),
        now_local.minute(),
        now_local.second()
    );

    // 计算前一天的时间
    let prev_day_utc = now_utc - 1.days();
    let prev_day_local = now_local - 1.days();

    println!("PREV_DAY_UTC: {}", prev_day_utc);
    println!("PREV_DAY_LOCAL: {}", prev_day_local);

    // 获取本周星期一的日期和零点的时间戳
    let this_monday_utc = now_utc
        .replace_day(now_utc.day() - now_utc.weekday().number_days_from_monday())
        .unwrap()
        .replace_time(time!(0:00));

    let this_monday_local = now_local
        .replace_day(now_local.day() - now_local.weekday().number_days_from_monday())
        .unwrap()
        .replace_time(time!(0:00));

    println!("THIS_MONDAY_UTC: {}", this_monday_utc);
    println!(
        "THIS_MONDAY_TIMESTAMP_UTC: {}",
        this_monday_utc.unix_timestamp()
    );

    println!("THIS_MONDAY_LOCAL: {}", this_monday_local);
    println!(
        "THIS_MONDAY_TIMESTAMP_LOCAL: {}",
        this_monday_local.unix_timestamp()
    );

    // 格式化
    println!(
        "NOW_DAY_FORMAT_UTC: {}",
        now_utc
            .format(
                &format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")
                    .unwrap()
            )
            .unwrap()
    );

    println!(
        "NOW_DAY_FORMAT_LOCAL: {}",
        now_local
            .format(
                &format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")
                    .unwrap()
            )
            .unwrap()
    );
}
