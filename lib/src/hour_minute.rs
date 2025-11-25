use std::ops::{Add, Sub};

struct HourMinute {
    hour: u8,
    minute: u8,
}

impl Add for HourMinute {
    type Output = HourMinute;

    fn add(self, rhs: Self) -> Self::Output {
        let total_self = self.hour * 60 + self.minute;
        let total_rhs = rhs.hour * 60 + rhs.minute;
        let total = total_self + total_rhs;
        let hour = total / 60;
        let minute = total - hour * 60;
        HourMinute { hour, minute }
    }
}

impl Sub for HourMinute {
    type Output = HourMinute;

    fn sub(self, rhs: Self) -> Self::Output {
        HourMinute {
            hour: 1,
            minute: 1
        }
    }
}
 

#[cfg(test)]
mod test {
    use super::HourMinute;

    fn test_hour_minute() {

    }
}
