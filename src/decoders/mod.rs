pub mod gpx;
pub mod json;
pub mod errors;

use std::str::FromStr;
use chrono::{DateTime, Utc};
use crate::decoders::{json::*, gpx::*, errors::*};

type RecordResult = Result<SpaceTimeRecord, DecoderError>;
#[derive(Debug)]
pub struct SpaceTimeRecord {
    pub points: Vec<SpaceTimePoint>,
    pub file_format: FileFormat,
}

type PointsResult = Result<Vec<SpaceTimePoint>, DecoderError>;
#[derive(Debug)]
pub struct SpaceTimePoint {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
}

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
                Ok(SpaceTimeRecord {points, file_format: format})
            },
            Err(e) => Err(e),
        }
    }

    pub fn get_points(&self) -> &[SpaceTimePoint] {
        &self.points
    }

    fn get_point_at_index(&self, index: usize) -> Option<&SpaceTimePoint> {
        self.points.get(index)
    }

    fn total_points(&self) -> usize {
        self.points.len()
    }

    fn get_file_format(&self) -> &FileFormat {
        &self.file_format
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
                    "endTime": "2015-01-25T09:11:16.547-08:00",
                    "startTime": "2015-01-24T20:16:07.217-08:00",
                    "visit": {
                        "hierarchyLevel": "0",
                        "topCandidate": {
                            "probability": "0.999080",
                            "semanticType": "Unknown",
                            "placeID": "R3DACT3D",
                            "placeLocation": "geo:35.456789,-120.567890"
                        },
                        "probability": "0.780000"
                    }
                },
                {
                    "endTime" : "2016-06-11T10:59:38.241-04:00",
                    "startTime" : "2016-06-11T10:37:59.047-04:00",
                    "activity" : {
                        "end" : "geo:38.765432,-76.987654",
                        "topCandidate" : {
                            "type" : "in passenger vehicle",
                            "probability" : "0.000000"
                        },
                        "distanceMeters" : "3146.000000",
                        "start" : "geo:38.812345,-77.039527"
                    }
                },
                {
                    "endTime" : "2017-08-15T08:00:00.000Z",
                    "startTime" : "2017-08-15T06:00:00.000Z",
                    "timelinePath" : [
                    {
                        "point" : "geo:37.654321,-122.345678",
                        "durationMinutesOffsetFromStartTime" : "56"
                    },
                    {
                        "point" : "geo:37.657890,-122.341234",
                        "durationMinutesOffsetFromStartTime" : "59"
                    }
                    ]
                }
            ]
        "#;

        let decoded_data = SpaceTimeRecord::new(json_content, FileFormat::Json).expect("Failed to parse JSON content");
        let points = decoded_data.get_points();
        assert_eq!(points.len(), 5);
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

        let decoded_data = SpaceTimeRecord::new(&gpx_content, FileFormat::Gpx).expect("Failed to parse GPX content");
        let points = decoded_data.get_points();
        assert_eq!(points.len(), 1);
        // Add more specific assertions based on the expected decoded data
    }
}