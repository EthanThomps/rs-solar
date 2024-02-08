use std::time::UNIX_EPOCH;

use crate::{
    julian::JD2NOON,
    kepler::{Body, Date, HourType, Time, TimeZone},
    orbit::{MeanMotion, Perihelion, SemiAxis},
};
use displaydoc::Display;
use strum::{AsRefStr, EnumProperty};

use super::EARTH_ROTATIONAL_PERIOD;

#[derive(Debug, Copy, Clone)]
/// This structure represents the fourth planet from the sun
pub struct Mars;

#[derive(Default, Debug, Copy, Clone, AsRefStr, EnumProperty)]
/// This structure represents the martian timezone
///
/// Offset is in 1 decisol, (-2.5 west, +2.5 east)
///
/// There is no DST on mars
///
/// 1 sol = 25 hours
/// 1 decisol = 2.5 hours
///
/// 12.5 + 12.5 = 25
/// MTC-5 to MTC+5 is 25 hours
pub enum Martian {
    #[strum(props(
        Code = "AMT",
        Name = "Amazonis Time",
        Offset = "-12.5",
        East = "-180",
        West = "-162"
    ))]
    /// Mars Coordinated Time - 5
    MTCn5,
    #[strum(props(
        Code = "OT",
        Name = "Olympus Time",
        Offset = "-10.0",
        East = "-162",
        West = "-126"
    ))]
    /// Mars Coordinated Time - 4
    MTCn4,
    #[strum(props(
        Code = "TT",
        Name = "Tharsis Time",
        Offset = "-7.5",
        East = "-126",
        West = "-90"
    ))]
    /// Mars Coordinated Time - 3
    MTCn3,
    #[strum(props(
        Code = "MT",
        Name = "Marineris Time",
        Offset = "-5.0",
        East = "-90",
        West = "-54"
    ))]
    /// Mars Coordinated Time - 2
    MTCn2,
    #[strum(props(
        Code = "AGT",
        Name = "Argyre Time",
        Offset = "-2.5",
        East = "-54",
        West = "-18"
    ))]
    /// Mars Coordinated Time - 1
    MTCn1,
    #[default]
    #[strum(props(
        Code = "NT",
        Name = "Noachis Time",
        Offset = "0.0",
        East = "-18",
        West = "18"
    ))]
    /// Mars Coordinated Time
    MTC,
    #[strum(props(
        Code = "ABT",
        Name = "Arabia Time",
        Offset = "2.5",
        East = "18",
        West = "54"
    ))]
    /// Mars Coordinated Time + 1
    MTCp1,
    #[strum(props(
        Code = "HT",
        Name = "Hellas Time",
        Offset = "5.0",
        East = "54",
        West = "90"
    ))]
    /// Mars Coordinated Time + 2
    MTCp2,
    #[strum(props(
        Code = "UT",
        Name = "Utopia Time",
        Offset = "7.5",
        East = "90",
        West = "126"
    ))]
    /// Mars Coordinated Time + 3
    MTCp3,
    #[strum(props(
        Code = "ET",
        Name = "Elysium Time",
        Offset = "10.0",
        East = "126",
        West = "162"
    ))]
    /// Mars Coordinated Time + 4
    MTCp4,
    #[strum(props(
        Code = "ACT",
        Name = "Arcadia Time",
        Offset = "12.5",
        East = "162",
        West = "180"
    ))]
    /// Mars Coordinated Time + 5
    MTCp5,
}

impl Body for Mars {
    /// A.D 1975 December 19, 04:00:00.3
    fn epoch(&self) -> f64 {
        2.442765667e6
    }

    fn orbital_eccentricity(&self) -> f64 {
        0.0934
    }

    fn orbital_period(&self) -> f64 {
        668.6
    }

    fn rotational_period(&self) -> f64 {
        88_775.245
    }

    fn perihelion(&self) -> Perihelion {
        Perihelion {
            month: (468.5, 514.6),
            ls: (240.0, 270.0),
            perihelion: 251.0,
        }
    }

    fn semimajor(&self) -> f64 {
        1.52
    }

    fn semiminor(&self) -> f64 {
        SemiAxis(self.semimajor()).minor(self.orbital_eccentricity())
    }

    fn mean_motion(&mut self, day: f64) -> f64 {
        MeanMotion::by(
            &mut MeanMotion,
            day,
            self.perihelion(),
            self.orbital_period(),
        )
    }

    fn to_date(&mut self, julian_date: f64) -> Date {
        Date::default().compute(
            julian_date,
            self.epoch(),
            self.rotational_period(),
            self.perihelion(),
            self.semimajor(),
            self.orbital_eccentricity(),
            self.orbital_period(),
        )
    }

    fn to_time(&mut self, date: Date) -> Time {
        Time::default().compute()
    }
}

impl TimeZone for Martian {
    /// Body Earth Ratio
    ///
    /// * body_rotational_period / earth_rotational_period
    ///
    /// Body Moon Ratio
    ///
    /// * moon_rotational_period / body_rotational_period (host planet of the exact moon)
    ///
    fn new(&self) -> Time {
        let millis = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Unix Epoch to function")
            .as_millis() as f64;

        let jd_ut = 2_440_587.5 + (millis / EARTH_ROTATIONAL_PERIOD * 1000.0);
        let jd_tt = jd_ut + (37.0 + 32.184) / EARTH_ROTATIONAL_PERIOD;
        let jd2000_t = jd_tt - JD2NOON;
        let mars_earth_ratio = 1.027491252_f64;
        let midday = 44_796.0_f64;
        let alignment = 0.00096_f64;
        let msx0 = jd2000_t - 4.5;
        let msd = (msx0 / mars_earth_ratio) + midday - alignment;
        // let mtc = (24.0 * msd) % 24.0;
        let fh = msd.fract(); // Fractional Hour
        let mut hour = (24.0 * fh).floor();
        let fm = (24.0 * fh).fract();
        let minute = (60.0 * fm).floor();
        let second = 60.0 * (60.0 * fm).fract();
        let hour_type = HourType::default().new(
            hour as u8
                + self
                    .get_str("Offset")
                    .unwrap()
                    .parse::<f64>()
                    .expect("Offset to be established") as u8,
        );

        match hour as u8 > 24 {
            true => hour = 0.0,
            false => (),
        }

        println!(
            "East: {:?}, West: {:?}",
            self.get_str("East").unwrap(),
            self.get_str("West").unwrap()
        );
        
        Time {
            hour: hour as i32,
            minute: minute as u8,
            second: second as u8,
            code: self.get_str("Code").unwrap().to_string(),
            name: self.get_str("Name").unwrap().to_string(),
            offset_name: self.as_ref().to_string(),
            hour_type: hour_type,
        }
    }
}
