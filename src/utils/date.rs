use time::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};
use time::macros::format_description;

pub enum OperateEnum {
    Later,
    Earlier,
}

// 获取UTC时间
pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

// 获取本地时间
pub fn now_local(hours: i8,
                 minutes: i8,
                 seconds: i8, ) -> OffsetDateTime {
    let utc_now = now_utc();
    let offset = UtcOffset::from_hms(hours, minutes, seconds).unwrap();
    utc_now.to_offset(offset)
}

// 将常见的格式化字符串映射到 time 的格式描述符
/*fn get_format_description(format: &str) -> Vec<FormatItem<'static>> {
    let mut format_map = HashMap::new();
    format_map.insert("yyyy", "[year]");
    format_map.insert("MM", "[month]");
    format_map.insert("dd", "[day]");
    format_map.insert("HH", "[hour]");
    format_map.insert("mm", "[minute]");
    format_map.insert("ss", "[second]");

    let mut format_str = format.to_string();
    for (key, value) in format_map {
        format_str = format_str.replace(key, value);
    }
    // 使用动态生成的格式描述符
    format_description!(format_str.as_str())
}*/

/*pub fn format_dynamic(datetime: OffsetDateTime, format: &str) -> String {
    let format_description = get_format_description(format);
    datetime.format(&format_description).unwrap()
}*/

// 时间格式化
pub fn format(date_time: OffsetDateTime) -> String {
    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    date_time.format(format).unwrap()
}

// 解析时间字符串
pub fn parse(time_str: &str) -> OffsetDateTime {
    let formats = [
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
        format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]"), // ISO 8601 风格
    ];
    for format in &formats {
        if let Ok(datetime) = PrimitiveDateTime::parse(time_str, format) {
            // 转换为 UTC 偏移的 OffsetDateTime
            return datetime.assume_utc();
        }
    }
    panic!("Failed to parse the provided time string: {}", time_str);
}

// 计算时间间隔
pub fn duration(date_time: OffsetDateTime, duration: Duration, operate_enum: OperateEnum) -> OffsetDateTime {
    match operate_enum {
        OperateEnum::Later => {
            date_time + duration
        }
        OperateEnum::Earlier => {
            date_time - duration
        }
    }
}

/**
 *获取时间差
 */
pub fn offset(start: OffsetDateTime, end: OffsetDateTime) -> Duration {
    start - end
}

// 创建一个日期
pub fn date(year: i32, month: Month, day: u8) -> Date {
    Date::from_calendar_date(year, month, day).unwrap()
}

// 创建一个时间
pub fn time(hour: u8, minute: u8, second: u8) -> Time {
    Time::from_hms(hour, minute, second).unwrap()
}

// 合并日期和时间
pub fn primitive_date_time(date: Date, time: Time) -> PrimitiveDateTime {
    PrimitiveDateTime::new(date, time)
}

pub fn offset_to_primitive(offset_datetime: OffsetDateTime) -> PrimitiveDateTime {
    // 从 OffsetDateTime 提取日期和时间，构造 PrimitiveDateTime
    PrimitiveDateTime::new(offset_datetime.date(), offset_datetime.time())
}

pub fn primitive_to_offset(primitive_datetime: PrimitiveDateTime, offset: UtcOffset) -> OffsetDateTime {
    // 使用给定的偏移量将 PrimitiveDateTime 转换为 OffsetDateTime
    primitive_datetime.assume_offset(offset)
}

#[cfg(test)]
mod tests {
    use time::Duration;

    use super::*;

    #[test]
    fn test_now_utc() {
        let utc_time = now_utc();
        assert!(utc_time.year() > 2023);
        println!("UTC Time: {}", format(utc_time));
    }

    #[test]
    fn test_now_local() {
        // 东八区
        let local_time = now_local(8, 0, 0);
        assert!(local_time.year() > 2023);
        println!("Local Time: {}", format(local_time));
    }

    #[test]
    fn test_format() {
        let utc_time = now_utc();
        let formatted_time = format(utc_time);
        assert!(formatted_time.contains("-"));
        println!("Formatted UTC Time: {}", formatted_time);
    }

    #[test]
    fn test_parse() {
        let time_str = "2024-09-20 12:30:00";
        let parsed_time = parse(time_str);
        assert_eq!(parsed_time.year(), 2024);
        assert_eq!(parsed_time.month(), Month::September);
        assert_eq!(parsed_time.day(), 20);
        println!("Parsed Time: {}", format(parsed_time));
    }

    #[test]
    fn test_duration_later() {
        let utc_time = now_utc();
        let duration_5_min = Duration::minutes(5);
        let new_time = duration(utc_time, duration_5_min, OperateEnum::Later);
        assert_eq!(new_time - utc_time, duration_5_min);
        println!("Time after 5 minutes: {}", format(new_time));
    }

    #[test]
    fn test_duration_earlier() {
        let utc_time = now_utc();
        let duration_5_min = Duration::minutes(5);
        let new_time = duration(utc_time, duration_5_min, OperateEnum::Earlier);
        assert_eq!(utc_time - new_time, duration_5_min);
        println!("Time 5 minutes earlier: {}", format(new_time));
    }

    #[test]
    fn test_offset() {
        let time1 = now_utc();
        let time2 = duration(time1, Duration::minutes(10), OperateEnum::Later);
        let diff = offset(time1, time2);
        assert_eq!(diff, Duration::minutes(-10));
    }

    #[test]
    fn test_date_and_time() {
        let date = date(2024, Month::September, 20);
        let time = time(12, 30, 0);
        let primitive_datetime = primitive_date_time(date, time);
        assert_eq!(primitive_datetime.date().year(), 2024);
        assert_eq!(primitive_datetime.time().hour(), 12);
        println!("Primitive DateTime: {}", primitive_datetime);
    }

    #[test]
    fn test_offset_to_primitive() {
        let offset_datetime = now_utc();
        let primitive_datetime = offset_to_primitive(offset_datetime);
        assert_eq!(primitive_datetime.date(), offset_datetime.date());
        assert_eq!(primitive_datetime.time(), offset_datetime.time());
    }

    #[test]
    fn test_primitive_to_offset() {
        let primitive_datetime = PrimitiveDateTime::new(
            Date::from_calendar_date(2024, Month::September, 20).unwrap(),
            Time::from_hms(12, 30, 0).unwrap(),
        );
        let offset = UtcOffset::from_hms(8, 0, 0).unwrap();
        let offset_datetime = primitive_to_offset(primitive_datetime, offset);
        assert_eq!(offset_datetime.offset(), offset);
        println!("Offset DateTime: {}", format(offset_datetime));
    }
}