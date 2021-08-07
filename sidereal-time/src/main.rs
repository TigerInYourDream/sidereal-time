use chrono::{Utc, TimeZone, FixedOffset, Datelike, Timelike};
use num::traits::pow;

fn main() {
    println!("sidereal time!");

    // let birthday = Utc.ymd(1987, 4, 10).and_hms(19, 21, 00);
    let birthday = Utc.ymd(2016, 11, 2).and_hms(21, 17, 30);
    let timezone = FixedOffset::east(0);
    birthday.with_timezone(&timezone);

    let mut year = birthday.year() as f64;
    let mut month = birthday.month() as f64;
    let day = birthday.day() as f64;
    let hour = birthday.hour() as f64;
    let minute = birthday.minute() as f64;
    let second = birthday.second() as f64;

    if month < 2f64 {
        year = year - 1f64;
        month = month + 12f64;
    }

    let jd = (&365.25 * year).floor() + (year / 400f64).floor() - (&year / 100f64).floor() + (30.59 * (month - 2f64)).floor()
        + day + 1721088.5 + hour / 24f64 + minute / 1440f64 + second / 86400f64;
    println!("jd: {:?}", jd);

    let t = (jd - 2451545.0) / 36525f64;
    println!("t: {:?}", t);
    let g = 280.46061837 + 360.98564736629 * (jd - 2451545.0) + 0.000387933 * pow(t, 2) - pow(t, 3) / 38710000f64;
    println!("g: {:?}", g);
    let mut res = g % 360f64;
    if res < 0f64 {
        res = 360f64 + res;
    }
    println!("res {:?}°", res);

    res = res / 15f64;
    println!("The sidereal hour is {:?}h", res);

    let hour = res.floor();
    let minute = ((res - hour) * 60f64).floor();
    let second = (((res - hour) * 60f64) - minute) * 60f64;
    println!("The sidereal time is {:?}h{:?}m{:?}s", hour, minute, second);
}