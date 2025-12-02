use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
struct HourMinute {
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
    pub fn try_new(hour: u8, minute: u8) -> Option<Self> {
        if hour <= 23 && minute <= 59 {
            Some(HourMinute { hour, minute })
        } else {
            None
        }
    }

    pub fn new(hour: u8, minute: u8) -> Self {
        Self::try_new(hour, minute).unwrap()
    }

    pub fn fmt_24(&self) -> String {
        self.to_string()
    }

    pub fn fmt_ampm(&self) -> String {
        if self.hour > 11 {
            format!("{:02}:{:02} P.M.", self.hour - 12, self.minute)
        } else {
            format!("{:02}:{:02} A.M.", self.hour, self.minute)            
        }
    }
}

//impl Add for HourMinute {
//    type Output = HourMinute;
//
//    fn add(self, rhs: Self) -> Self::Output {
//        let total_self = self.hour * 60 + self.minute;
//        let total_rhs = rhs.hour * 60 + rhs.minute;
//        let total = total_self + total_rhs;
//        let hour = total / 60;
//        let minute = total - hour * 60;
//        HourMinute { hour, minute }
//    }
//}
//
//impl Sub for HourMinute {
//    type Output = HourMinute;
//
//    fn sub(self, rhs: Self) -> Self::Output {
//        HourMinute {
//            hour: 1,
//            minute: 1
//        }
//    }
//}
 
#[cfg(test)]
mod test {
    use super::HourMinute;
 
    #[test]
    fn default() {
        assert_eq!(HourMinute::default(), HourMinute { hour: 0, minute: 0 });
    }

    #[test]
    fn try_new_some() {
        assert_eq!(HourMinute::try_new(0, 0), Some(HourMinute { hour: 0, minute: 0 }));
        assert_eq!(HourMinute::try_new(23, 59), Some(HourMinute { hour: 23, minute: 59 }));
        assert_eq!(HourMinute::try_new(1, 2), Some(HourMinute { hour: 1, minute: 2 }));
        assert_eq!(HourMinute::try_new(3, 4), Some(HourMinute { hour: 3, minute: 4 }));
    }

    #[test]
    fn try_new_none() {
        assert_eq!(HourMinute::try_new(0, 60), None);
        assert_eq!(HourMinute::try_new(24, 0), None);
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
    }

    #[test]
    fn fmt_24() {
        assert_eq!(HourMinute::new(0, 0).fmt_24(), "00:00");
        assert_eq!(HourMinute::new(3, 10).fmt_24(), "03:10");
        assert_eq!(HourMinute::new(12, 34).fmt_24(), "12:34");
        assert_eq!(HourMinute::new(18, 8).fmt_24(), "18:08");
        assert_eq!(HourMinute::new(23, 59).fmt_24(), "23:59");
    }

    #[test]
    fn fmt_ampm() {
        assert_eq!(HourMinute::new(0, 0).fmt_ampm(), "00:00 A.M.");
        assert_eq!(HourMinute::new(3, 10).fmt_ampm(), "03:10 A.M.");
        assert_eq!(HourMinute::new(11, 51).fmt_ampm(), "11:51 A.M.");
        assert_eq!(HourMinute::new(11, 59).fmt_ampm(), "11:59 A.M.");
        assert_eq!(HourMinute::new(12, 00).fmt_ampm(), "00:00 P.M.");
        assert_eq!(HourMinute::new(12, 01).fmt_ampm(), "00:01 P.M.");
        assert_eq!(HourMinute::new(12, 34).fmt_ampm(), "00:34 P.M.");
        assert_eq!(HourMinute::new(18, 8).fmt_ampm(), "06:08 P.M.");
        assert_eq!(HourMinute::new(23, 59).fmt_ampm(), "11:59 P.M.");
    }
}
