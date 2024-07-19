use super::{errors::DecoderError, PointsResult, SpaceTimePoint};
use std::str::FromStr;
use serde::Deserialize;
use serde_json;
use chrono::{Duration, DateTime, Utc};

#[derive(Deserialize)]
#[serde(untagged)]
pub enum JsonRecord
{
    JsonEntries(Vec<JsonEntry>),
    TimelineObjects(TimeLineObjects),
    LocationEntries(LocationEntries),
}

trait IntoSpaceTimePoints {
    fn to_space_time_points(&self) -> PointsResult;
}

impl Into<PointsResult> for JsonRecord {
    fn into(self) -> PointsResult {
        let mut space_time_points: Vec<SpaceTimePoint> = Vec::new();
        match self {
            JsonRecord::JsonEntries(entries) => {
                space_time_points.reserve(entries.len()); // at minimum, there are as many points as entries
                for entry in entries {
                    space_time_points.append(&mut entry.to_space_time_points()?);
                }
            },
            JsonRecord::TimelineObjects(timeline_objects) => {
                space_time_points.reserve(timeline_objects.timeline_objects.len()); // at minimum, there are as many points as entries
                for entry in &timeline_objects.timeline_objects {
                    space_time_points.append(&mut entry.to_space_time_points()?);
                }
            },
            JsonRecord::LocationEntries(location_entries) => {
                space_time_points.reserve_exact(location_entries.locations.len()); // We know there are exactly this many entries
                space_time_points.append(&mut location_entries.to_space_time_points()?);
            },
            _ => return Err(DecoderError::DeserializeError("Unsupported Type".to_string()))
        }
        Ok(space_time_points)
    }
}

impl FromStr for JsonRecord {
    type Err = DecoderError;
    fn  from_str(content: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(content)?)
    }
}

type TimestampRfc3339 = String;
type GeoLocationE7 = i64;

#[derive(Deserialize)]
pub struct TimeLineObjects
{
    #[serde(rename = "timelineObjects")]
    timeline_objects: Vec<TimelineObject>
}

#[derive(Deserialize)]
struct TimelineObject
{
    #[serde(rename = "placeVisit")]
    place_visit: Option<PlaceVisit>,
    #[serde(rename = "activitySegment")]
    activity_segment: Option<ActivitySegment>,
}

#[derive(Deserialize)]
struct PlaceVisit
{
    location: Location,
    duration: JsonDuration
}

#[derive(Deserialize)]
struct ActivitySegment
{
    #[serde(rename = "startLocation")]
    start_location: Location,
    #[serde(rename = "endLocation")]
    end_location: Location,
    duration: JsonDuration,
    #[serde(rename = "waypointPath")]
    waypoint_path: WaypointPath
}

#[derive(Deserialize)]
struct Location
{
    #[serde(rename = "latitudeE7", alias = "latE7")]
    latitude_e7: GeoLocationE7,
    #[serde(rename = "longitudeE7", alias = "lngE7")]
    longitude_e7: GeoLocationE7
}

#[derive(Deserialize)]
struct JsonDuration
{
    #[serde(rename = "startTimestamp")]
    start_timestamp: TimestampRfc3339,
    #[serde(rename = "endTimestamp")]
    end_timestamp: TimestampRfc3339
}

#[derive(Deserialize)]
struct WaypointPath
{
    waypoints: Vec<Location>
}

enum TimelineType
{
    PlaceVisit,
    ActivitySegment
}

impl IntoSpaceTimePoints for TimelineObject
{
    fn to_space_time_points(&self) -> PointsResult {
        match self.get_timeline_type()? {
            TimelineType::PlaceVisit => self.parse_place_visit(),
            TimelineType::ActivitySegment => self.parse_activity_segment()
        }
    }
}

impl TimelineObject
{
    fn parse_place_visit(&self) -> PointsResult {
        let place_visit = match self.place_visit.as_ref() {
            Some(place_visit) => place_visit,
            None => return Err(DecoderError::EmptyEntryError("TimelineObject classified as PlaceVisit but was empty".to_string()))
        };
        Ok(vec![SpaceTimePoint
        {
            latitude: parse_geolocation_e7(&place_visit.location.latitude_e7)?,
            longitude: parse_geolocation_e7(&place_visit.location.longitude_e7)?,
            start_time: parse_timestamp_str(&place_visit.duration.start_timestamp)?,
            end_time: parse_timestamp_str(&place_visit.duration.end_timestamp)?
        }])
    }

    fn parse_activity_segment(&self) -> PointsResult {
        let activity_segment = match self.activity_segment.as_ref() {
            Some(activity_segment) => activity_segment,
            None => return Err(DecoderError::EmptyEntryError("TimelineObject classified as ActivitySegment but was empty".to_string()))
        };

        let start_time = parse_timestamp_str(&activity_segment.duration.start_timestamp)?;
        let end_time = parse_timestamp_str(&activity_segment.duration.end_timestamp)?;
        
        let num_points = 2 + activity_segment.waypoint_path.waypoints.len();
        let segment_duration = (end_time - start_time) / (num_points as i32);

        let mut space_time_points = Vec::with_capacity(num_points);
        let mut last_point_end_time = start_time;
        for i in 0..num_points {
            let point_end_time = if i == num_points - 1 {
                end_time
            }
            else {
                last_point_end_time + segment_duration
            };
            
            let waypoint = if i == 0 {
                &activity_segment.start_location
            } else if i == num_points - 1 {
                &activity_segment.end_location
            } else {
                &activity_segment.waypoint_path.waypoints[i - 1]
                // &activity_segment.start_location
            };
            let latitude = parse_geolocation_e7(&waypoint.latitude_e7)?;
            let longitude = parse_geolocation_e7(&waypoint.longitude_e7)?;

            space_time_points.push(SpaceTimePoint{
                start_time: last_point_end_time,
                end_time: point_end_time,
                latitude, longitude
            });

            last_point_end_time = point_end_time;
        }

        Ok(space_time_points)
    }

    fn get_timeline_type(&self) -> Result<TimelineType, DecoderError> {
        if self.place_visit.is_some() {
            Ok(TimelineType::PlaceVisit)
        } else if self.activity_segment.is_some() {
            Ok(TimelineType::ActivitySegment)
        } else {
            Err(DecoderError::EmptyEntryError("Empty TimelineObject".to_string()))
        }
    }
}

#[derive(Deserialize)]
pub struct LocationEntries
{
    #[serde(rename = "locations")]
    locations: Vec<LocationEntry>
}

#[derive(Deserialize)]
struct LocationEntry
{
    #[serde(rename = "latitudeE7")]
    latitude_e7: GeoLocationE7,
    #[serde(rename = "longitudeE7")]
    longitude_e7: GeoLocationE7,
    #[serde(rename = "timestampMs")]
    timestamp_ms: Option<String>,
    timestamp: Option<TimestampRfc3339>
}

impl IntoSpaceTimePoints for LocationEntries
{
    fn to_space_time_points(&self) -> PointsResult {
        let mut space_time_points = Vec::with_capacity(self.locations.len());

        for location in self.locations.iter().rev() {
            let timestamp = location.get_timestamp()?;
    
            space_time_points.push(SpaceTimePoint {
                latitude: parse_geolocation_e7(&location.latitude_e7)?,
                longitude: parse_geolocation_e7(&location.longitude_e7)?,
                start_time: timestamp,
                end_time: timestamp
            });
        }

        Ok(space_time_points)
    }
}

impl LocationEntry {
    fn get_timestamp(&self) -> Result<DateTime<Utc>, DecoderError> {
        if let Some(timestamp_ms) = self.timestamp_ms.as_ref()
        {
            match DateTime::from_timestamp_millis(timestamp_ms.parse()?)
            {
                Some(timestamp) => return Ok(timestamp),
                None => return Err(DecoderError::TimeParseError(format!("Error Converting {} TimestampMs to DateTime", timestamp_ms)))
            };
        }
        else if let Some(timestamp) = self.timestamp.as_ref()
        {
            return Ok(parse_timestamp_str(timestamp)?);
        }

        return Err(DecoderError::TimeParseError(format!("Location {}, {} missing timestamp", self.latitude_e7, self.longitude_e7)));
    }
}

type GeoLocation = String;
#[derive(Deserialize, Debug)]
pub struct JsonEntry {
    #[serde(rename = "startTime")]
    start_time: TimestampRfc3339,
    #[serde(rename = "endTime")]
    end_time: TimestampRfc3339,
    visit: Option<Visit>,
    #[serde(rename = "timelinePath")]
    timeline_path: Option<Vec<TimelinePath>>,
    activity: Option<Activity>,
}

#[derive(Deserialize, Debug)]
struct Visit {
    #[serde(rename = "topCandidate")]
    top_candidate: TopCandidate,
}

#[derive(Deserialize, Debug)]
struct TopCandidate {
    #[serde(rename = "placeLocation")]
    place_location: GeoLocation,
}

#[derive(Deserialize, Debug)]
struct TimelinePath {
    point: GeoLocation,
    #[serde(rename = "durationMinutesOffsetFromStartTime")]
    duration_minutes_offset_from_start_time: String,
}

#[derive(Deserialize, Debug)]
struct Activity {
    start: GeoLocation,
    end: GeoLocation,
}

enum EntryType {
    Visit,
    TimelinePath,
    StartEnd,
}

impl IntoSpaceTimePoints for JsonEntry {
    fn to_space_time_points(&self) -> PointsResult {
        match self.get_entry_type()? {
            EntryType::Visit => self.parse_visit(),
            EntryType::TimelinePath => self.parse_timeline_path(),
            EntryType::StartEnd => self.parse_start_end_entry(),
        }
    }
}

impl JsonEntry {
    fn parse_visit(&self) -> PointsResult {
        let start_time = parse_timestamp_str(&self.start_time)?;
        let end_time = parse_timestamp_str(&self.end_time)?;
        let geo_location = match self.visit.as_ref() {
            Some(visit) => JsonEntry::parse_geolocation(&visit.top_candidate.place_location)?,
            None => return Err(DecoderError::EmptyEntryError(format!("Entry {:?} classified as Visit but was empty", self.start_time))),
        };
        let point = SpaceTimePoint{start_time, end_time, latitude: geo_location.0, longitude: geo_location.1};
        Ok(vec![point])
    }

    fn parse_timeline_path(&self) -> PointsResult {
        let path_start_time = parse_timestamp_str(&self.start_time)?;
        let path_end_time = parse_timestamp_str(&self.end_time)?;

        let timeline = match self.timeline_path.as_ref() {
            Some(timeline) => timeline,
            None => return Err(DecoderError::EmptyEntryError(format!("Entry {:?} classified as TimelinePath but was empty", self.start_time))),
        };

        let timeline_len = timeline.len();
        let mut space_time_points = Vec::with_capacity(timeline_len); // at minimum, there are as many points as timelinePaths
        let mut last_point_end_time = path_start_time;
        for (i, timeline_point) in timeline.iter().enumerate() {
            let geo_location = JsonEntry::parse_geolocation(&timeline_point.point)?;
            let point_end_time = if i + 1 == timeline_len {
                    path_end_time
                } else {
                    let start_time_minutes_offset: i64 = timeline_point.duration_minutes_offset_from_start_time.parse()?;
                    path_start_time + Duration::minutes(start_time_minutes_offset)
                };
            space_time_points.push(SpaceTimePoint{start_time: last_point_end_time, end_time: point_end_time, latitude: geo_location.0, longitude: geo_location.1});
            last_point_end_time = point_end_time;
        }
        Ok(space_time_points)
    }

    fn parse_start_end_entry(&self) -> PointsResult {
        let activity_start_time = parse_timestamp_str(&self.start_time)?;
        let activity_end_time = parse_timestamp_str(&self.end_time)?;
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
            Err(DecoderError::EmptyEntryError(format!("No Data Found for {:?} : {:?}", self.start_time, self)))
        }
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

/// Parse time strings tht follow the RFC3339 format
/// example: "2015-01-25T09:11:16.547-08:00"
/// example: "2017-08-15T08:00:00.000Z"
fn parse_timestamp_str(timestamp: &TimestampRfc3339) -> Result<DateTime<Utc>, DecoderError> {
    let dt = DateTime::parse_from_rfc3339(timestamp.as_str())?;
    Ok(dt.with_timezone(&Utc))
}

// Parse GeoLocationE7 numbers
// example: 374219999 -> 37.4219999
fn parse_geolocation_e7(geolocation_e7: &GeoLocationE7) -> Result<f64, DecoderError> {
    if geolocation_e7.to_string().len() <= 7 {
        return Err(DecoderError::GeoParseError(format!("Geographic Coordinate {} has too few digits", geolocation_e7)));
    }
    Ok((*geolocation_e7 as f64) / 1E7)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_timestamp_str() {
        let timestamp = "2015-01-25T09:11:16.547-08:00";
        let dt = parse_timestamp_str(&timestamp.to_string());
        assert!(dt.is_ok());
        assert_eq!(dt.unwrap().format("%Y-%m-%dT%H:%M:%S%.3f%:z").to_string(), "2015-01-25T17:11:16.547+00:00");
    }

    #[test]
    fn test_parse_geolocation_e7() {
        let geolocation_e7 = 374219999;
        let geolocation = parse_geolocation_e7(&geolocation_e7);
        assert!(geolocation.is_ok());
        assert_eq!(geolocation.unwrap(), 37.4219999);
    }
}