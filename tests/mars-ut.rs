#[cfg(test)]
mod tests {
    use rust_solar::{
        julian::jd2greg,
        kepler::Body,
        planets::mars::Mars,
    };

    #[test]
    pub fn mars_to_date() {
        let jd = 2440587.5;
        let date = Mars.to_date(jd);
        jd2greg(jd);
        println!("The date is {:?}", date);
    }

}
