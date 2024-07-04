use super::*;
use serde::Deserialize;
use serde_json;
use chrono::Duration;

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
    timeline_path: Option<Vec<TimelinePath>>,
    activity: Option<Activity>,
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
struct TimelinePath {
    point: GeoLocation,
    #[serde(rename = "durationMinutesOffsetFromStartTime")]
    duration_minutes_offset_from_start_time: String,
}

#[derive(Deserialize)]
struct Activity {
    start: GeoLocation,
    end: GeoLocation,
}

impl Into<PointsResult> for JsonRecord {
    fn into(self) -> PointsResult {
        let mut space_time_points = Vec::with_capacity(self.entries.len()); // at minimum, there are as many points as entries
        for entry in &self.entries {
            space_time_points.append(&mut entry.to_space_time_points()?);
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
        match self.get_entry_type()? {
            EntryType::Visit => self.parse_visit(),
            EntryType::TimelinePath => self.parse_timeline_path(),
            EntryType::StartEnd => self.parse_start_end_entry(),
        }
    }

    fn parse_visit(&self) -> PointsResult {
        let start_time = JsonEntry::parse_timestamp_str(&self.start_time)?;
        let end_time = JsonEntry::parse_timestamp_str(&self.end_time)?;
        let geo_location = match self.visit.as_ref() {
            Some(visit) => JsonEntry::parse_geolocation(&visit.top_candidate.place_location)?,
            None => return Err(DecoderError::EmptyEntryError(format!("Entry {:?} classified as Visit but was empty", self.start_time))),
        };
        let point = SpaceTimePoint{start_time, end_time, latitude: geo_location.0, longitude: geo_location.1};
        Ok(vec![point])
    }

    fn parse_timeline_path(&self) -> PointsResult {
        let path_start_time = JsonEntry::parse_timestamp_str(&self.start_time)?;
        let path_end_time = JsonEntry::parse_timestamp_str(&self.end_time)?;

        let timeline = match self.timeline_path.as_ref() {
            Some(timeline) => timeline,
            None => return Err(DecoderError::EmptyEntryError(format!("Entry {:?} classified as TimelinePath but was empty", self.start_time))),
        };

        let timeline_len = timeline.len();
        let mut space_time_points = Vec::with_capacity(timeline_len); // at minimum, there are as many points as timelinePaths
        let mut last_point_end_time = path_start_time;
        for (i, timeline_point) in timeline.iter().enumerate() {
            let geo_location = JsonEntry::parse_geolocation(&timeline_point.point)?;
            let point_end_time = match i + 1 {
                timeline_len => path_end_time,
                _ => {
                    let start_time_minutes_offset: i64 = timeline_point.duration_minutes_offset_from_start_time.parse()?;
                    path_start_time + Duration::minutes(start_time_minutes_offset)
                }
            };
            space_time_points.push(SpaceTimePoint{start_time: last_point_end_time, end_time: point_end_time, latitude: geo_location.0, longitude: geo_location.1});
            last_point_end_time = point_end_time;
        }
        Ok(space_time_points)
    }

    fn parse_start_end_entry(&self) -> PointsResult {
        let activity_start_time = JsonEntry::parse_timestamp_str(&self.start_time)?;
        let activity_end_time = JsonEntry::parse_timestamp_str(&self.end_time)?;
        let activity_mid_time = activity_start_time + (activity_end_time - activity_start_time) / 2;

        let (start_geo_location, end_geo_location) = match self.activity.as_ref() {
            Some(activity) => (JsonEntry::parse_geolocation(&activity.start)?, JsonEntry::parse_geolocation(&activity.end)?),
            None => return Err(DecoderError::EmptyEntryError(format!("Entry {:?} classified as StartEnd Entry but was empty", self.start_time))),
        };

        let start_point = SpaceTimePoint{start_time: activity_start_time, end_time: activity_mid_time, latitude: start_geo_location.0, longitude: start_geo_location.1};
        let end_point = SpaceTimePoint{start_time: activity_mid_time, end_time: activity_end_time, latitude: end_geo_location.0, longitude: end_geo_location.1};
        Ok(vec![start_point, end_point])
    }

    fn get_entry_type(&self) -> Result<EntryType, DecoderError> {
        if self.activity.is_some() {
            Ok(EntryType::StartEnd)
        } else if self.visit.is_some() {
            Ok(EntryType::Visit)
        } else if self.timeline_path.is_some() {
            Ok(EntryType::TimelinePath)
        } else {
            Err(DecoderError::EmptyEntryError(format!("No Data Found for {:?}", self.start_time)))
        }
    }

    /// Parse time strings
    /// example: "2015-01-25T09:11:16.547-08:00"
    /// example: "2017-08-15T08:00:00.000Z"
    fn parse_timestamp_str(timestamp: &str) -> Result<DateTime<Utc>, DecoderError> {
        let dt = DateTime::parse_from_rfc3339(timestamp)?;
        Ok(dt.with_timezone(&Utc))
    }

    /// Parse geolocation string
    /// example: "geo:37.4219999,-122.0840576"
    fn parse_geolocation(geolocation: &GeoLocation) -> Result<(f64, f64), DecoderError> {
        let parts: Vec<&str> = geolocation.split(',').collect();
        if parts.len() == 2 {
            let latitude = parts[0].trim_start_matches("geo:").parse::<f64>().ok();
            let longitude = parts[1].parse::<f64>().ok();
            match (latitude, longitude) {
                (Some(lat), Some(lon)) => return Ok((lat, lon)),
                _ => ()
            }
        }
        Err(DecoderError::GeoParseError(format!("Unable to parse location string {}", geolocation)))
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
