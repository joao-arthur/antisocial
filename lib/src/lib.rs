use std::time::{Duration, Instant};

// struct ApplicationSettings {
//     id: String,
//     managed: bool,
//     allowed_continuous_use: HourMinute,
//     allowed_amount_of_sessions: u64
// }

// struct GeneralSettings {
//     manages
//     notifies
//     daily_target    
//     notification_period
// }

struct Use {
    pub from: u64,
    pub to: u64,
    pub total: u64
}

enum UseEventType {
    Enter,
    Left
}

struct UseEvent {
    pub application: String,
    pub t: UseEventType,
    pub timestamp: u64
}


trait Provider {

}

struct User {

}

struct Device {

}

#[cfg(test)]
mod test {
    use super::HourMinute;

    fn test_hour_minute() {

    }
}
