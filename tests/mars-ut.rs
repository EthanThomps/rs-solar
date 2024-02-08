#[cfg(test)]
mod tests {
    use rust_solar::{
        julian::jd2greg,
        kepler::{Body, TimeZone},
        planets::mars::{Mars, Martian},
    };

    #[test]
    pub fn mars_to_date() {
        let jd = 2440587.5;
        let date = Mars.to_date(jd);
        jd2greg(jd);
        println!("The date is {:?}", date);
    }

    #[test]
    pub fn mars_to_time() {
        let time = Martian::MTCp5.new();

        println!("Time now: {:?}", time);
    }

}

