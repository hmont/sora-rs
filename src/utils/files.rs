use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Tz;

pub async fn gen_filename(length: usize) -> String {
    let rng = thread_rng();
    let filename: String = rng.sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    filename
}

pub async fn timestamp_str(timestamp: i64) -> String {
    let est: Tz = "America/New_York".parse().unwrap();
    let datetime: DateTime<Utc> = Utc.timestamp_opt(timestamp, 0).unwrap();
    let est_datetime: DateTime<Tz> = datetime.with_timezone(&est);
    est_datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}