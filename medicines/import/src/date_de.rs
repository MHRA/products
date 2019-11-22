use chrono::{DateTime, ParseResult, TimeZone, Utc};
use serde::{self, Deserialize, Deserializer};

const DATETIME: &str = "%D %R";

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    convert(s).map_err(serde::de::Error::custom)
}

fn convert(s: String) -> ParseResult<DateTime<Utc>> {
    if let Ok(x) = Utc.datetime_from_str(&s, DATETIME) {
        Ok(x)
    } else {
        Utc.datetime_from_str(&format!("{} 00:00", &s), DATETIME)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_datetime() {
        assert_eq!(
            convert("1/19/15 8:51".to_string()),
            Ok(Utc.ymd(2015, 1, 19).and_hms(8, 51, 0))
        );
    }
    #[test]
    fn test_date() {
        assert_eq!(
            convert("1/19/15".to_string()),
            Ok(Utc.ymd(2015, 1, 19).and_hms(0, 0, 0))
        );
    }
}
