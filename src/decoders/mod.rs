pub mod gpx;
pub mod json;
pub mod errors;

use std::str::FromStr;
use crate::model::{SpaceTimePoint, SpaceTimeRecord};
use crate::decoders::{json::JsonRecord, gpx::GpxRecords, errors::*};

type RecordResult = Result<SpaceTimeRecord, DecoderError>;
type PointsResult = Result<Vec<SpaceTimePoint>, DecoderError>;

#[derive(Debug)]
pub enum FileFormat {
    Json,
    Gpx,
}

impl SpaceTimeRecord {
    pub fn new(content: &str, format: FileFormat) -> RecordResult {
        let points: PointsResult = match format {
            FileFormat::Json => JsonRecord::from_str(content)?.into(),
            FileFormat::Gpx => GpxRecords::from_str(content)?.into(),
        };
        match points {
            Ok(points) => 
            {
                debug_assert!(points.windows(2).all(|w| w[0].end_time <= w[1].start_time)); // Ensure points are sorted and don't overlap - removed in release mode
                Ok(SpaceTimeRecord {points})
            },
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests;