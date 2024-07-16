use crate::model::{SpaceTimeRecord, SpaceTimePoint};
use rstar::{RTree, RTreeObject, AABB, PointDistance};

impl RTreeObject for SpaceTimePoint {
    type Envelope = AABB<[f64; 4]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.latitude, self.longitude, self.start_time.timestamp() as f64, self.end_time.timestamp() as f64])
    }
}

impl SpaceTimePoint {
    const TEMPORAL_WEIGHT: f64 = 0.5;
    const SPATIAL_WEIGHT: f64 = 1.0 - Self::TEMPORAL_WEIGHT;
}

impl PointDistance for SpaceTimePoint {
    fn distance_2(&self, point: &[f64; 4]) -> f64 {
        let spatial_distance = self.haversine_distance(point[0], point[1]);
        let temporal_distance = self.temporal_distance(point[2], point[3]);

        Self::SPATIAL_WEIGHT * spatial_distance.powi(2) + Self::TEMPORAL_WEIGHT * temporal_distance.powi(2)
    }

    fn contains_point(&self, point: &[f64; 4]) -> bool {
        return self.latitude == point[0] && self.longitude == point[1] && self.start_time.timestamp() as f64 == point[2] && self.end_time.timestamp() as f64 == point[3];
    }

    fn distance_2_if_less_or_equal(&self, point: &[f64; 4], max_distance_2: f64) -> Option<f64> {
        let temporal_component = self.temporal_distance(point[2], point[3]).powi(2) * Self::TEMPORAL_WEIGHT;
        if temporal_component > max_distance_2 {
            return None;
        }

        let spatial_component = self.euclidean_distance(point[0], point[1]).powi(2) * Self::SPATIAL_WEIGHT;
        if spatial_component > max_distance_2 {
            return None;
        }

        Some(self.distance_2(point))
    }
}
