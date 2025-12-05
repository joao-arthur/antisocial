use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
pub struct HourMinute {
    pub hour: u8,
    pub minute: u8,
}

impl Default for HourMinute {
    fn default() -> Self {
        HourMinute { hour: 0, minute: 0 }
    }
}

impl std::fmt::Display for HourMinute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}

impl HourMinute {
    pub fn new(hour: u8, minute: u8) -> Self {
        HourMinute { hour, minute }
    }
}

impl Add for HourMinute {
    type Output = HourMinute;

    fn add(self, rhs: Self) -> Self::Output {
        let total_self = u16::from(self.hour) * 60 + u16::from(self.minute);
        let total_rhs = u16::from(rhs.hour) * 60 + u16::from(rhs.minute);
        let total = total_self + total_rhs;
        let hour = total / 60;
        let minute = total - hour * 60;
        HourMinute { hour: hour as u8, minute: minute as u8 }
    }
}

impl Sub for HourMinute {
    type Output = HourMinute;

    fn sub(self, rhs: Self) -> Self::Output {
        let total_self = u16::from(self.hour) * 60 + u16::from(self.minute);
        let total_rhs = u16::from(rhs.hour) * 60 + u16::from(rhs.minute);
        let total = total_self - total_rhs;
        let hour = total / 60;
        let minute = total - hour * 60;
        HourMinute { hour: hour as u8, minute: minute as u8 }
    }
}
 
#[cfg(test)]
mod test {
    use super::HourMinute;
 
    #[test]
    fn default() {
        assert_eq!(HourMinute::default(), HourMinute { hour: 0, minute: 0 });
    }

    #[test]
    fn new() {
        assert_eq!(HourMinute::new(0, 0), HourMinute { hour: 0, minute: 0 });
        assert_eq!(HourMinute::new(23, 59), HourMinute { hour: 23, minute: 59 });
        assert_eq!(HourMinute::new(1, 2), HourMinute { hour: 1, minute: 2 });
        assert_eq!(HourMinute::new(3, 4), HourMinute { hour: 3, minute: 4 });
    }

    #[test]
    fn to_string() {
        assert_eq!(HourMinute::new(0, 0).to_string(), "00:00");
        assert_eq!(HourMinute::new(3, 10).to_string(), "03:10");
        assert_eq!(HourMinute::new(12, 34).to_string(), "12:34");
        assert_eq!(HourMinute::new(18, 8).to_string(), "18:08");
        assert_eq!(HourMinute::new(23, 59).to_string(), "23:59");
    }

    #[test]
    fn add() {
        assert_eq!(HourMinute::new(0, 0) + HourMinute::new(0, 0), HourMinute::new(0, 0));
        assert_eq!(HourMinute::new(1, 0) + HourMinute::new(0, 0), HourMinute::new(1, 0));
        assert_eq!(HourMinute::new(0, 0) + HourMinute::new(0, 1), HourMinute::new(0, 1));
        assert_eq!(HourMinute::new(1, 0) + HourMinute::new(0, 1), HourMinute::new(1, 1));
        assert_eq!(HourMinute::new(8, 3) + HourMinute::new(2, 7), HourMinute::new(10, 10));
        assert_eq!(HourMinute::new(8, 20) + HourMinute::new(1, 50), HourMinute::new(10, 10));
        assert_eq!(HourMinute::new(8, 59) + HourMinute::new(1, 11), HourMinute::new(10, 10));
        assert_eq!(HourMinute::new(8, 59) + HourMinute::new(0, 1), HourMinute::new(9, 0));
        assert_eq!(HourMinute::new(11, 29) + HourMinute::new(12, 30), HourMinute::new(23, 59));
        assert_eq!(HourMinute::new(0, 0) + HourMinute::new(23, 59), HourMinute::new(23, 59));
        assert_eq!(HourMinute::new(23, 59) + HourMinute::new(0, 0), HourMinute::new(23, 59));
    }

    #[test]
    fn sub() {
        assert_eq!(HourMinute::new(0, 0) - HourMinute::new(0, 0), HourMinute::new(0, 0));
        assert_eq!(HourMinute::new(1, 0) - HourMinute::new(0, 0), HourMinute::new(1, 0));
        assert_eq!(HourMinute::new(1, 0) - HourMinute::new(1, 0), HourMinute::new(0, 0));
        assert_eq!(HourMinute::new(0, 1) - HourMinute::new(0, 1), HourMinute::new(0, 0));
        assert_eq!(HourMinute::new(10, 10) - HourMinute::new(2, 7), HourMinute::new(8, 3));
        assert_eq!(HourMinute::new(10, 10) - HourMinute::new(1, 50), HourMinute::new(8, 20));
        assert_eq!(HourMinute::new(10, 10) - HourMinute::new(1, 11), HourMinute::new(8, 59));
        assert_eq!(HourMinute::new(9, 0) - HourMinute::new(0, 1), HourMinute::new(8, 59));
        assert_eq!(HourMinute::new(23, 59) - HourMinute::new(11, 29), HourMinute::new(12, 30));
        assert_eq!(HourMinute::new(23, 59) - HourMinute::new(23, 59), HourMinute::new(0, 0));
    }
}
