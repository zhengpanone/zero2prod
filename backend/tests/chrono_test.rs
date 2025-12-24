use chrono::{
	Duration, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc,
};

#[test]
fn test_chrono_new_test() {
	let date = NaiveDate::from_ymd_opt(2025, 12, 24).unwrap();
	let time = NaiveTime::from_hms_opt(21, 50, 50).unwrap();
	let datetime = NaiveDateTime::new(date, time);
	println!("Date: {}", date);
	println!("Time: {}", time);
	println!("DateTime: {}", datetime);
}

#[test]
fn test_chrono_format_test() {
	let start = NaiveDate::from_ymd_opt(2025, 12, 24)
		.and_then(|date| {
			NaiveTime::from_hms_opt(21, 50, 50)
				.map(|time| NaiveDateTime::new(date, time))
		})
		.unwrap();
	let duratin = Duration::hours(3) + Duration::minutes(30);
	let end = start + duratin;
	println!("Start Time: {}", start.format("%Y-%m-%d %H:%M:%S"));
	println!("End Time: {}", end.format("%Y-%m-%d %H:%M:%S"));
}

#[test]
fn test_chrono_utc_test() {
	let utc_time = Utc::now();
	// 东八区（UTC+8）
	let local_time =
		utc_time.with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap());
	println!("UTC Time: {}", utc_time.to_rfc2822());
	let local_time = Local::now();
	println!("UTC Time: {}", utc_time.to_rfc2822());
	println!("Local Time: {}", local_time.to_rfc2822());
}

#[test]
fn test_parse_date() {
	let result =
		NaiveDateTime::parse_from_str("2025-12-24 22:10:10", "%Y-%m-%d %H:%M:%S");
	match result {
		Ok(datetime) => println!("Parse Date Success: {}", datetime),
		Err(e) => println!("Parse Date Error: {}", e),
	}
}
