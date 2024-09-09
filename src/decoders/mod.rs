pub mod gpx;
pub mod json;
pub mod errors;

use std::str::FromStr;
use crate::compute::spacetime::{SpaceTimePoint, SpaceTimeRecord};
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
mod tests {
    use super::*;

    #[test]
    fn test_json_decoder_json_entry() {
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
        let points = decoded_data.points;
        assert_eq!(points.len(), 5);
        // Add more specific assertions based on the expected decoded data
    }

    #[test]
    fn test_json_decoder_timeline_object() {
        let json_content = r#"
            {
            "timelineObjects": [
                {
                "activitySegment": {
                    "startLocation": {
                    "latitudeE7": 391122849,
                    "longitudeE7": -848646131,
                    "sourceInfo": {
                        "deviceTag": 1931280707
                    }
                    },
                    "endLocation": {
                    "latitudeE7": 391361546,
                    "longitudeE7": -848427170,
                    "sourceInfo": {
                        "deviceTag": 1931280707
                    }
                    },
                    "duration": {
                    "startTimestamp": "2022-07-02T17:05:39.060Z",
                    "endTimestamp": "2022-07-02T17:10:29.999Z"
                    },
                    "distance": 3279,
                    "activityType": "IN_PASSENGER_VEHICLE",
                    "confidence": "HIGH",
                    "activities": [
                    {
                        "activityType": "IN_PASSENGER_VEHICLE",
                        "probability": 0
                    }
                    ],
                    "waypointPath": {
                    "waypoints": [
                        {
                        "latE7": 391122779,
                        "lngE7": -848646163
                        },
                        {
                        "latE7": 391354293,
                        "lngE7": -848428039
                        }
                    ],
                    "source": "INFERRED",
                    "distanceMeters": 3996.997910197294,
                    "travelMode": "DRIVE",
                    "confidence": 1
                    },
                    "editConfirmationStatus": "CONFIRMED"
                }
                },
                {
                "placeVisit": {
                    "location": {
                    "latitudeE7": 391364127,
                    "longitudeE7": -848425601,
                    "semanticType": "TYPE_ALIASED_LOCATION",
                    "sourceInfo": {
                        "deviceTag": 1931280707
                    },
                    "locationConfidence": 94.2616,
                    "calibratedProbability": 82.84482
                    },
                    "duration": {
                    "startTimestamp": "2022-07-02T17:10:29.999Z",
                    "endTimestamp": "2022-07-02T21:53:54.024Z"
                    },
                    "placeConfidence": "HIGH_CONFIDENCE",
                    "centerLatE7": 391360208,
                    "centerLngE7": -848424538,
                    "visitConfidence": 91,
                    "editConfirmationStatus": "NOT_CONFIRMED",
                    "locationConfidence": 80,
                    "placeVisitType": "SINGLE_PLACE",
                    "placeVisitImportance": "MAIN"
                }
                }
            ]
            }
        "#;
        let decoded_data = SpaceTimeRecord::new(json_content, FileFormat::Json).expect("Failed to parse JSON content");
        let points = decoded_data.points;
        assert_eq!(points.len(), 5);
    }

    #[test]
    fn test_json_decoder_location_entry() {
        let json_content = r#"
            {
            "locations": [
                {
                "timestampMs": "1545965352966",
                "latitudeE7": 442367395,
                "longitudeE7": -764915858,
                "accuracy": 10,
                "velocity": 1,
                "heading": 147,
                "altitude": 112,
                "verticalAccuracy": 24
                },
                {
                "timestampMs": "1545950209000",
                "latitudeE7": 442376572,
                "longitudeE7": -764913977,
                "accuracy": 10,
                "velocity": 0,
                "altitude": 79,
                "verticalAccuracy": 16
                },
                {
                "timestampMs": "1545949883998",
                "latitudeE7": 442350519,
                "longitudeE7": -764875263,
                "accuracy": 5,
                "velocity": 1,
                "heading": 291,
                "altitude": 96,
                "verticalAccuracy": 3
                },
                {
                "timestampMs": "1545949718083",
                "latitudeE7": 442350467,
                "longitudeE7": -764838955,
                "accuracy": 10,
                "velocity": 0,
                "heading": 154,
                "altitude": 85,
                "verticalAccuracy": 16
                },
                {
                "timestampMs": "1545936860513",
                "latitudeE7": 442349325,
                "longitudeE7": -764826912,
                "accuracy": 65,
                "altitude": 92,
                "verticalAccuracy": 14
                }
            ]
            }
        "#;
        let decoded_data = SpaceTimeRecord::new(json_content, FileFormat::Json).expect("Failed to parse JSON content");
        let points = decoded_data.points;
        assert_eq!(points.len(), 5);
    }

    // GPX Decoding is Broken but not part of the MVP. Will fix as a TODO item
    // #[test]
    // fn test_gpx_decoder() {
    //     let gpx_content = r#"
    //     <?xml version="1.0" encoding="UTF-8"?>
    //     <gpx version="1.1">
    //         <trk>
    //             <trkseg>
    //                 <trkpt lat="37.7749" lon="-122.4194">
    //                     <time>2023-06-29T10:00:00Z</time>
    //                 </trkpt>
    //             </trkseg>
    //         </trk>
    //     </gpx>
    //     "#.to_string();

    //     let decoded_data = SpaceTimeRecord::new(&gpx_content, FileFormat::Gpx).expect("Failed to parse GPX content");
    //     let points = decoded_data.points;
    //     assert_eq!(points.len(), 1);
    //     // Add more specific assertions based on the expected decoded data
    // }
}