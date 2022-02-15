use chrono::{Duration, NaiveTime};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Clock {
    time: NaiveTime,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (h, m) = time_parser(hours, minutes);
        Clock {
            time: NaiveTime::from_hms(h, m, 0),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            time: self.time + Duration::minutes(minutes as i64),
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.time.format("%H:%M").to_string())
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for Clock {}

fn time_parser(hours: i32, minutes: i32) -> (u32, u32) {
    let m_q = minutes / 60;
    let mut m_r = minutes % 60;
    let mut h_r = (hours + m_q) % 24;

    if m_r < 0 {
        m_r = 60 + m_r;
        h_r -= 1;
    }
    if h_r < 0 {
        h_r = 24 + h_r
    }

    (h_r as u32, m_r as u32)
}
