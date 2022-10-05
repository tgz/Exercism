use std::fmt;

#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let temp_minutes = hours * 60 + minutes;
        let mut minutes = temp_minutes % 60;
        let mut hours = (temp_minutes/60) % 24;
        if minutes < 0 {
            hours -= 1;
        }
        
        if hours < 0 {
            hours = hours + 24
        }

        if minutes < 0 {
            minutes = minutes + 60
        }

        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let current_minutes = self.hours * 60 + self.minutes + minutes;
        Clock::new(0, current_minutes)
    }
}

impl fmt::Display for Clock {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fix = self.add_minutes(0);
        write!(f, "{:02}:{:02}", fix.hours, fix.minutes)
   }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fix = self.add_minutes(0);
        write!(f, "{:02}:{:02}", fix.hours, fix.minutes)
    }
}


