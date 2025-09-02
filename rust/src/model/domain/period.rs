use chrono::Datelike;
use chrono::NaiveDate;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone)]
pub struct Period(i32, i32);

impl fmt::Display for Period {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{:02}", self.0, self.1)
    }
}

impl From<NaiveDate> for Period {
    fn from(date: NaiveDate) -> Self {
        Period(date.year(), date.month() as i32)
    }
}

impl Period {
    pub fn new(y: i32, m: i32) -> Period {
        Period(y, m)
    }
    pub fn year_period(&self) -> i32 {
        self.0 * 100 + self.1
    }
    pub fn next_period(&self) -> Period {
        if self.1 == 12 {
            Period::new(self.0 + 1, 1)
        } else {
            Period::new(self.0, self.1 + 1)
        }
    }
    pub fn prev_period(&self) -> Period {
        if self.1 == 1 {
            Period::new(self.0 - 1, 12)
        } else {
            Period::new(self.0, self.1 - 1)
        }
    }
    fn is_leap_year(&self) -> bool {
        let year = self.0;
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }

    pub fn last_day(&self) -> u32 {
        let month = self.1;
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year() {
                    29
                } else {
                    28
                }
            }
            _ => panic!("Invalid month: {}", month),
        }
    }

    pub fn first_date(&self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.0, self.1 as u32, 1).unwrap()
    }
    pub fn last_date(&self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.0, self.1 as u32, self.last_day()).unwrap()
    }

    pub fn date(&self, day: u32) -> NaiveDate {
        let day = if day < 1 {
            1
        } else if day > self.last_day() {
            self.last_day()
        } else {
            day
        };
        NaiveDate::from_ymd_opt(self.0, self.1 as u32, day).unwrap()
    }
    
    pub fn first_period(&self) -> Period {
        Period::new(self.0, 1)
    }

    pub fn last_period(&self) -> Period {
        Period::new(self.0, 12)
    }
    
    pub fn year(&self) -> u32 {
        self.0 as u32
    }

    pub fn month(&self) -> u32 {
        self.1 as u32
    }
}
