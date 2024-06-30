pub mod gpx;
pub mod json;

use std::str::FromStr;
use chrono::{DateTime, Utc};
use enum_dispatch::enum_dispatch;
use crate::decoders::{gpx::*, json::*};

#[derive(Debug)]
pub struct SpaceTimePoint {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub longitude: f64,
    pub latitude: f64,

}

#[enum_dispatch]
pub trait LocationRecords {
    fn get_points(&self) -> &[SpaceTimePoint];
    fn get_point_at_index(&self, index: usize) -> Option<&SpaceTimePoint>;
    fn total_points(&self) -> usize;
}

#[enum_dispatch(LocationRecords,FromStr)]
#[derive(Debug)]
pub enum RecordType where {
    JsonRecords,
    GpxRecords,
}

pub enum FileFormat {
    Json,
    Gpx,
}

pub fn to_record(format: FileFormat, content: &str) -> Result<RecordType, Box<dyn std::error::Error>> {
    match format {
        FileFormat::Json => Ok(RecordType::from(JsonRecords::from_str(content)?)),
        FileFormat::Gpx => Ok(RecordType::from(GpxRecords::from_str(content)?)),
    }
}

pub fn to_format(record: &RecordType) -> FileFormat {
    match record {
        RecordType::JsonRecords(_) => FileFormat::Json,
        RecordType::GpxRecords(_) => FileFormat::Gpx,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_decoder() {
        let json_content = r#"
    [
        {
            "start_time": "2023-03-25T09:00:00Z",
            "end_time": "2023-03-25T09:15:00Z",
            "longitude": 125.6,
            "latitude": 10.1
        }
    ]
    "#.to_string();

        let decoded_data = to_record(FileFormat::Json, &json_content).expect("Failed to parse JSON content");
        let points = decoded_data.get_points();
        assert_eq!(points.len(), 1);
        // Add more specific assertions based on the expected decoded data
    }

    #[test]
    fn test_gpx_decoder() {
        let gpx_content = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <gpx version="1.1">
            <trk>
                <trkseg>
                    <trkpt lat="37.7749" lon="-122.4194">
                        <time>2023-06-29T10:00:00Z</time>
                    </trkpt>
                </trkseg>
            </trk>
        </gpx>
        "#.to_string();

        let decoded_data = to_record(FileFormat::Gpx, &gpx_content).expect("Failed to parse GPX content");
        let points = decoded_data.get_points();
        assert_eq!(points.len(), 1);
        // Add more specific assertions based on the expected decoded data
    }
}