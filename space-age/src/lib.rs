#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

const EARTH_SEC: u64 = 31_557_600;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / Self::ORBIT_SEC
    }

    const ORBIT_SEC: f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const ORBIT_SEC: f64 = 0.240_846_7 * EARTH_SEC as f64;
}
impl Planet for Venus {
    const ORBIT_SEC: f64 = 0.615_197_26 * EARTH_SEC as f64;
}
impl Planet for Earth {
    const ORBIT_SEC: f64 = 1.0 * EARTH_SEC as f64;
}
impl Planet for Mars {
    const ORBIT_SEC: f64 = 1.880_815_8 * EARTH_SEC as f64;
}
impl Planet for Jupiter {
    const ORBIT_SEC: f64 = 11.862_615 * EARTH_SEC as f64;
}
impl Planet for Saturn {
    const ORBIT_SEC: f64 = 29.447_498 * EARTH_SEC as f64;
}
impl Planet for Uranus {
    const ORBIT_SEC: f64 = 84.016_846 * EARTH_SEC as f64;
}
impl Planet for Neptune {
    const ORBIT_SEC: f64 = 164.791_320 * EARTH_SEC as f64;
}
