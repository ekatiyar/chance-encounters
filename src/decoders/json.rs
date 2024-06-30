use std::str::FromStr;
use crate::decoders::{LocationRecords, SpaceTimePoint};
use serde::Deserialize;
use serde_json;

#[derive(Debug)]
pub struct JsonRecords {
    points: Vec<SpaceTimePoint>,
}

#[derive(Deserialize)]
struct JsonPoint {
    start_time: String,
    end_time: String,
    longitude: f64,
    latitude: f64,
}

impl FromStr for JsonRecords {
    type Err = Box<dyn std::error::Error>;
    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let json_points: Vec<JsonPoint> = serde_json::from_str(content)?;
        let points = json_points
            .into_iter()
            .map(|jp| Ok(SpaceTimePoint{
                start_time: jp.start_time.parse()?,
                end_time: jp.end_time.parse()?,
                longitude: jp.longitude,
                latitude: jp.latitude,
            }))
            .collect::<Result<Vec<_>, Self::Err>>();
        Ok(JsonRecords {points: points?})
    }
}

impl LocationRecords for JsonRecords {
    fn get_points(&self) -> &[SpaceTimePoint] {
        &self.points
    }

    fn get_point_at_index(&self, index: usize) -> Option<&SpaceTimePoint> {
        self.points.get(index)
    }

    fn total_points(&self) -> usize {
        self.points.len()
    }
}