use super::*;
use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
pub struct JsonRecord
{
    entries: Vec<JsonEntry>,
}

type GeoLocation = String;
#[derive(Deserialize)]
struct JsonEntry {
    #[serde(rename = "startTime")]
    start_time: String,
    #[serde(rename = "endTime")]
    end_time: String,
    visit: Option<Visit>,
    #[serde(rename = "timelinePath")]
    timeline_path: Option<Vec<TimeLinePath>>,
    start: Option<GeoLocation>,
    end: Option<GeoLocation>,
}

#[derive(Deserialize)]
struct Visit {
    #[serde(rename = "topCandidate")]
    top_candidate: TopCandidate,
}

#[derive(Deserialize)]
struct TopCandidate {
    #[serde(rename = "placeLocation")]
    place_location: GeoLocation,
}

#[derive(Deserialize)]
struct TimeLinePath {
    point: GeoLocation,
    #[serde(rename = "durationMinutesOffsetFromStartTime")]
    duration_minutes_offset_from_start_time: String,
}

impl Into<PointsResult> for JsonRecord {
    fn into(self) -> PointsResult {
        let mut space_time_points = Vec::with_capacity(self.entries.len());
        for entry in &self.entries {
            space_time_points.extend(entry.to_space_time_points()?);
        }
        Ok(space_time_points)
    }
}

enum EntryType {
    Visit,
    TimelinePath,
    StartEnd,
}

impl JsonEntry {
    pub fn to_space_time_points(&self) -> PointsResult {
        // if self.start.is_some() || self.end.is_some() {
        //     let mut geolocations = Vec::new();
        //     if let Some(start) = &self.start {
        //         geolocations.push(start.clone());
        //     }
        //     if let Some(end) = &self.end {
        //         geolocations.push(end.clone());
        //     }
        //     return geolocations;
        // }
        // if let Some(visit) = &self.visit {
        //     if let Some(top_candidate) = &visit.top_candidate {
        //         return vec![top_candidate.place_location.clone()];
        //     }
        // }
        // if let Some(timeline_path) = &self.timeline_path {
        //     return timeline_path.iter().map(|tlp| tlp.point.clone()).collect();
        // }
        Ok(vec![])
    }

    fn get_entry_type(&self) -> Result<EntryType, DecoderError> {
        if self.start.is_some() && self.end.is_some() {
            Ok(EntryType::StartEnd)
        } else if self.visit.is_some() {
            Ok(EntryType::Visit)
        } else if self.timeline_path.is_some() {
            Ok(EntryType::TimelinePath)
        } else {
            Err(DecoderError::EmptyEntryError(format!("No Data Found for {:?}", self.start_time)))
        }
    }

    //// Parse geolocation string
    /// example: "geo:37.4219999,-122.0840576"
    fn parse_geolocation(geolocation: &GeoLocation) -> Option<(f64, f64)> {
        let parts: Vec<&str> = geolocation.split(',').collect();
        if parts.len() == 2 {
            let latitude = parts[0].trim_start_matches("geo:").parse::<f64>().ok();
            let longitude = parts[1].parse::<f64>().ok();
            match (latitude, longitude) {
                (Some(lat), Some(lon)) => Some((lat, lon)),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl FromStr for JsonRecord {
    type Err = DecoderError;
    fn from_str(content: &str) -> Result<Self, Self::Err> {
        Ok(JsonRecord{  entries: serde_json::from_str(content)? })
    }
}

impl From<serde_json::Error> for DecoderError {
    fn from(err: serde_json::Error) -> Self {
        DecoderError::DeserializeError(err.to_string())
    }
}
