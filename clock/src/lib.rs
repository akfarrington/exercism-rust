use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock::crunch_minutes(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::crunch_minutes(self.hours * 60 + self.minutes + minutes)
    }

    pub fn crunch_minutes(minutes: i32) -> Self {
        let mut new_hours = minutes / 60;
        let mut new_minutes = minutes % 60;

        // when add_minutes passes negative number of minutes
        while new_minutes < 0 {
            new_minutes += 60;
            new_hours -= 1;
        }
        // when negative add_minutes makes new_hours negative
        while new_hours < 0 {
            new_hours += 24;
        }
        // when Clock::new() passes a number of hours over 24
        while new_hours >= 24 {
            new_hours -= 24;
        }

        Clock {
            hours: new_hours,
            minutes: new_minutes,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:02}:{:02}", self.hours, self.minutes)
    }
}
