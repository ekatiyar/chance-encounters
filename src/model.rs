use chrono::{DateTime, Utc};

pub const EARTH_RADIUS_KM: f64 = 6371.0;

/// Great-circle distance (km) between two lat/lon pairs. Free function so it can
/// be called on raw coordinates (e.g. the R-tree's `[lat, lon, ...]` arrays)
/// without a [`SpaceTimePoint`] to hang a method off of.
pub fn haversine_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat1 - lat2).to_radians();
    let delta_lon = (lon1 - lon2).to_radians();

    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    EARTH_RADIUS_KM * c
}

#[derive(Debug)]
pub struct SpaceTimeRecord {
    pub points: Vec<SpaceTimePoint>,
}

#[derive(Debug)]
pub struct SpaceTimePoint {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
}

impl SpaceTimePoint {
    pub fn haversine_distance(&self, latitude: f64, longitude: f64) -> f64 {
        haversine_km(self.latitude, self.longitude, latitude, longitude)
    }

    pub fn temporal_distance(&self, start_time: f64, end_time: f64) -> f64 {
        if self.temporal_overlap(start_time, end_time) {
            return 0.0
        }
        start_time.max(self.start_time.timestamp() as f64) - end_time.min(self.end_time.timestamp() as f64)
    }

    fn temporal_overlap(&self, start_time: f64, end_time: f64) -> bool {
        let self_start = self.start_time.timestamp() as f64;
        let self_end = self.end_time.timestamp() as f64;

        // Standard half-open interval overlap.
        start_time < self_end && self_start < end_time
    }
}

#[cfg(test)]
mod tests;