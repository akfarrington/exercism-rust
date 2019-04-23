use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{
            hours,
            minutes,
        }.correct_time()
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock{
            hours: self.hours,
            minutes: self.minutes + minutes,
        }.correct_time()
    }

    pub fn correct_time(&self) -> Clock{
        // first break down the time to minutes
        let total_minutes: i32 = self.hours * 60 + self.minutes;

        let mut correct_time = Clock{
            hours: total_minutes / 60,
            minutes: total_minutes % 60,
        };

        // if the time is still messed up, this fixes it
        while correct_time.minutes < 0 {
            correct_time.minutes += 60;
            correct_time.hours -= 1;
        }
        while correct_time.hours < 0 {
            correct_time.hours += 24;
        }
        while correct_time.hours >= 24 {
            correct_time.hours -= 24;
        }

        correct_time
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:02}:{:02}", self.hours, self.minutes)
    }
}