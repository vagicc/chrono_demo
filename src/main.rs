use chrono::{
    Date, DateTime, Datelike, Duration, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime,
    Timelike, Utc,
};

fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
    date_time.checked_sub_signed(Duration::days(1))
}

fn main() {
    println!("执行日期检查和时间计算!");
    let now = Utc::now(); //UTC当前时间
    println!("UTC当前时间：{}", now);

    let almost_three_weeks_from_now = now
        .checked_sub_signed(Duration::weeks(2))
        .and_then(|in_2weeks| in_2weeks.checked_add_signed(Duration::weeks(1)))
        .and_then(day_earlier);
    match almost_three_weeks_from_now {
        Some(x) => println!("{}", x),
        None => eprintln!("差不多三周时间出现溢出"),
    }
    // let almost_three_weeks_from_now2 = now.checked_sub_signed(Duration::weeks(3));
    // println!("{:?}", almost_three_weeks_from_now2);

    match now.checked_add_signed(Duration::max_value()) {
        Some(x) => println!("{}", x),
        None => eprintln!("我们不能用chrono来计算太阳系围绕银河系中心运行超过一圈的时间。"),
    }

    println!("时间的时区转换");
    let local_time = Local::now(); //获取本地时间
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc); //转换为UTC时间
    let china_timezone = FixedOffset::east(8 * 3600); //中国时间：UTC+8
    let rio_timezone = FixedOffset::west(2 * 3600); //UTC-2
    println!("本地时间:{}", local_time);
    println!("香港时间:{}", utc_time.with_timezone(&china_timezone));
    println!(
        "Time in Rio de Janeiro now is {}",
        utc_time.with_timezone(&rio_timezone)
    );

    println!(
        "通过 Timelike 获取当前 UTC DateTime 及其时/分/秒，通过 Datelike 获取其年/月/日/工作日"
    );
    let now = Utc::now();

    let (is_pm, hour) = now.hour12();
    println!(
        "The current UTC time is {:02}:{:02}:{:02} {}",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    );
    println!("从午夜开始已经有{}秒了", now.num_seconds_from_midnight());

    let (is_common_era, year) = now.year_ce();
    println!(
        "The current UTC date is {}-{:02}-{:02} {:?} ({})",
        year,
        now.month(),
        now.day(),
        now.weekday(),
        if is_common_era { "CE" } else { "BCE" }
    );

    println!(
        "And the Common Era began {} days ago",
        now.num_days_from_ce()
    );

    println!("日期和 UNIX 时间戳的互相转换");
    let date_time = NaiveDate::from_ymd(2017, 11, 12).and_hms(17, 33, 44);
    println!(
        "Number of seconds between 1970-01-01 00:00:00 and {} is {}.",
        date_time,
        date_time.timestamp()
    );

    let date_time_after_a_billion_seconds = NaiveDateTime::from_timestamp(1_000_000_000, 0);
    println!(
        "Date after a billion seconds since 1970-01-01 00:00:00 was {}.",
        date_time_after_a_billion_seconds
    );

    println!("日期和时间的格式化显示");
    let now = Utc::now();
    println!("UTC now is:{}", now);
    println!("Utc now in RFC 2822 is:{}", now.to_rfc2822());
    println!("Utc now in RFC 3339 is:{}", now.to_rfc3339());
    println!(
        "Utc now in a custom format is:{}",
        now.format("%a %b %e %T %Y")
    );

    println!("将字符串解析为 DateTime 结构体");
    let rffc2822 = DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200").unwrap();
    println!("Tue, 1 Jul 2003 10:52:37 +0200 = {}", rffc2822);
    let rfc3339 = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")
        .expect("时间转为DateTime结构体出错");
    println!("1996-12-19T16:39:57-08:00 = {}", rfc3339);
    let custom =
        DateTime::parse_from_str("5.8.1994 8:00 am +0000", "%d.%m.%Y %H:%M %P %z").unwrap();
    println!("{}", custom);
    //解析不带时区的日期和时间，请使用 NaiveDate、NaiveTime，以及 NaiveDateTime。
    let time_only = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S").unwrap();
    println!("{}", time_only);
    let date_only = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d").unwrap();
    println!("{}", date_only);
    let no_timezone=NaiveDateTime::parse_from_str("2015-09-05 23:56:04","%Y-%m-%d %H:%M:%S").unwrap();
    println!("{}",no_timezone);
}
