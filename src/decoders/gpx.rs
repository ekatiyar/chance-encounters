use std::str::FromStr;
use crate::decoders::{LocationRecords, SpaceTimePoint};
use quick_xml::de::from_str;
use serde::Deserialize;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct GpxRecords {
    points: Vec<SpaceTimePoint>,
}

#[derive(Debug, Deserialize)]
struct Gpx {
    trk: Vec<Track>,
}

#[derive(Debug, Deserialize)]
struct Track {
    trkseg: Vec<TrackSegment>,
}

#[derive(Debug, Deserialize)]
struct TrackSegment {
    trkpt: Vec<TrackPoint>,
}

#[derive(Debug, Deserialize)]
struct TrackPoint {
    #[serde(rename = "@lat")]
    lat: f64,
    #[serde(rename = "@lon")]
    lon: f64,
    time: String,
}

impl FromStr for GpxRecords {
    type Err = Box<dyn std::error::Error>;
    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let gpx: Gpx = from_str(content)?;
        let mut points = Vec::new();

        for track in gpx.trk {
            for segment in track.trkseg {
                for (i, point) in segment.trkpt.iter().enumerate() {
                    let start_time: DateTime<Utc> = point.time.parse()?;
                    let end_time = if i + 1 < segment.trkpt.len() {
                        segment.trkpt[i + 1].time.parse()?
                    } else {
                        start_time // Use the same time for the last point
                    };

                    points.push(SpaceTimePoint {
                        start_time,
                        end_time,
                        latitude: point.lat,
                        longitude: point.lon,
                    });
                }
            }
        }

        Ok(GpxRecords { points })
    }
}

impl LocationRecords for GpxRecords {
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