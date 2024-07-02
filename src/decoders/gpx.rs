use super::*;
use serde::Deserialize;
use chrono::{DateTime, Utc};
use quick_xml::{de, DeError};

#[derive(Debug, Deserialize)]
pub struct GpxRecords {
    trk: Vec<Track>,
}

#[derive(Debug, Deserialize)]
struct Gpx {
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

impl Into<PointsResult> for GpxRecords {
    fn into(self) -> PointsResult {
        let mut space_time_points = Vec::with_capacity(self.trk.len());
        for track in &self.trk {
            space_time_points.extend(track.to_space_time_points()?);
        }
        Ok(space_time_points)
    }
}

impl Track {
    pub fn to_space_time_points(&self) -> Result<Vec<SpaceTimePoint>, DecoderError> {
        let mut points = Vec::new();

        for segment in &self.trkseg {
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

        Ok(points)
    }
}

impl FromStr for GpxRecords {
    type Err = DecoderError;
    fn from_str(content: &str) -> Result<Self, Self::Err> {
        de::from_str(content)?
    }
}

impl From<quick_xml::DeError> for DecoderError {
    fn from(err: DeError) -> Self {
        DecoderError::DeserializeError(err.to_string())
    }
}