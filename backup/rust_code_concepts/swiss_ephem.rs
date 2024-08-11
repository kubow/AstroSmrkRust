
use swisseph::{CalcFlags, Date, Planet, SwissEph};

pub fn calculate_planet_position(date: Date, time: f64, planet: Planet) {
    // Create a Swiss Ephemeris instance
    let mut swiss_ephemeris = SwissEph::new();

    // Calculate the position of the specified celestial object at the specified date and time
    let flags = CalcFlags::empty();
    let position = swiss_ephemeris.calc_ut(date, time, planet, flags).unwrap();

    // Print the result
    println!("{:?} Position: {:?}", planet, position);
}
