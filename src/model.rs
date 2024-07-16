use chrono::{DateTime, Utc};

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
    const EARTH_RADIUS: f64 = 6371.0; // in kilometers

    pub fn euclidean_distance(&self, latitude: f64, longitude: f64) -> f64 {
        let delta_lat = (self.latitude - latitude).to_radians();
        let delta_lon = (self.longitude - longitude).to_radians();
        let avg_lat = ((self.latitude + latitude) / 2.0).to_radians();

        (Self::EARTH_RADIUS.powi(2) * (delta_lat.powi(2) + (avg_lat.cos() * delta_lon).powi(2))).sqrt()
    }

    pub fn haversine_distance(&self, latitude: f64, longitude: f64) -> f64 {

        let lat1_rad = self.latitude.to_radians();
        let lat2_rad = latitude.to_radians();
        let delta_lat = (self.latitude - latitude).to_radians();
        let delta_lon = (self.longitude - longitude).to_radians();

        let a = (delta_lat / 2.0).sin().powi(2) + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        Self::EARTH_RADIUS * c
    }

    pub fn temporal_distance(&self, start_time: f64, end_time: f64) -> f64 {
        if self.temporal_overlap(start_time, end_time) {
            return 0.0
        }
        start_time.max(self.start_time.timestamp() as f64) - end_time.min(self.end_time.timestamp() as f64)
    }

    fn temporal_overlap(&self, start_time: f64, end_time: f64) -> bool {
        let self_start = self.start_time.timestamp() as f64;
        let  self_end = self.end_time.timestamp() as f64;

        // We have an overlap if the start time of one interval is within the other interval
        (start_time > self_start && start_time < self_end) || (self_start > start_time && self_start < end_time)        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const ERROR: f64 = 0.001;
    const TIME0: DateTime<Utc> = DateTime::from_timestamp_nanos(0);

    #[test]
    fn test_euclidean_distance() {
        let point = SpaceTimePoint {
            start_time: TIME0,
            end_time: TIME0,
            latitude: 41.507483,
            longitude: -99.436554,
        };

        let distance = point.euclidean_distance(38.504048, -98.315949);
        assert!((distance - 347.338).abs() < ERROR, "Distance was actually {}", distance);
    }

    #[test]
    fn test_haversine_distance() {
        let point = SpaceTimePoint {
            start_time: TIME0,
            end_time: TIME0,
            latitude: 41.507483,
            longitude: -99.436554,
        };

        let distance = point.haversine_distance(38.504048, -98.315949);
        assert!((distance - 347.328).abs() < ERROR, "Distance was actually {}", distance);
    }

    #[test]
    fn test_temporal_distance_overlap() {
        let point = SpaceTimePoint {
            start_time: DateTime::from_timestamp(100, 0).unwrap(),
            end_time: DateTime::from_timestamp(1000, 0).unwrap(),
            latitude: 0.0,
            longitude: 0.0,
        };

        assert_eq!(point.temporal_distance(500.0, 600.0), 0.0);
        assert_eq!(point.temporal_distance(500.0, 1500.0), 0.0);
        assert_eq!(point.temporal_distance(TIME0.timestamp() as f64, 500.0), 0.0);
    }

    #[test]
    fn test_temporal_distance() {
        let point = SpaceTimePoint {
            start_time: DateTime::from_timestamp(100, 0).unwrap(),
            end_time: DateTime::from_timestamp(1000, 0).unwrap(),
            latitude: 0.0,
            longitude: 0.0,
        };

        assert_eq!(point.temporal_distance(1500.0, 2000.0), 500.0);
        assert_eq!(point.temporal_distance(TIME0.timestamp() as f64, 50.0), 50.0);
    } 
}