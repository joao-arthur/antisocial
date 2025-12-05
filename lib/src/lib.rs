mod hour_minute;
mod day_hour_minute;
mod day_hour_minute_period;

use std::time::{Duration, Instant};

use crate::{day_hour_minute::DayHourMinute, day_hour_minute_period::DayHourMinutePeriod, hour_minute::HourMinute};

struct ApplicationSettings {
    id: String,
    managed: bool,
    allowed_continuous_use: HourMinute,
    allowed_amount_of_sessions: u64
}

struct GeneralSettings {
    manages: bool,
    notifies: bool,
    daily_target: DayHourMinute, 
    notification_period: DayHourMinutePeriod
}

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

struct Permissions {
    access_apps_use: bool,
    access_apps_list: bool,
    notify: bool,
}

trait Provider {

}

struct User {

}

struct Device {

}
