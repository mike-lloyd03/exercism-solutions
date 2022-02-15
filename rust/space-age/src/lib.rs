#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! make_planet {
    ($name:ident, $ratio:expr) => {
        pub struct $name;
        impl Planet for $name {
            fn years_during(d: &Duration) -> f64 {
                d.seconds as f64 / 3600.0 / 24.0 / 365.25 / $ratio
            }
        }
    };
}

make_planet!(Mercury, 0.2408467);
make_planet!(Venus, 0.61519726);
make_planet!(Earth, 1.0);
make_planet!(Mars, 1.8808158);
make_planet!(Jupiter, 11.862615);
make_planet!(Saturn, 29.447498);
make_planet!(Uranus, 84.016846);
make_planet!(Neptune, 164.79132);
