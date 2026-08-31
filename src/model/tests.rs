use super::*;
const ERROR: f64 = 0.001;
const TIME0: DateTime<Utc> = DateTime::from_timestamp_nanos(0);

#[test]
fn test_haversine_distance() {
    let point = SpaceTimePoint {
        start_time: TIME0,
        end_time: TIME0,
        latitude: 41.507483,
        longitude: -99.436554,
    };

    let distance = point.haversine_distance(38.504048, -98.315949);
    assert!((distance - 347.328).abs() < ERROR, "Distance was actually {}", distance);
}

#[test]
fn test_temporal_distance_overlap() {
    let point = SpaceTimePoint {
        start_time: DateTime::from_timestamp(100, 0).unwrap(),
        end_time: DateTime::from_timestamp(1000, 0).unwrap(),
        latitude: 0.0,
        longitude: 0.0,
    };

    assert_eq!(point.temporal_distance(500.0, 600.0), 0.0);
    assert_eq!(point.temporal_distance(500.0, 1500.0), 0.0);
    assert_eq!(point.temporal_distance(TIME0.timestamp() as f64, 500.0), 0.0);
}

#[test]
fn test_temporal_distance() {
    let point = SpaceTimePoint {
        start_time: DateTime::from_timestamp(100, 0).unwrap(),
        end_time: DateTime::from_timestamp(1000, 0).unwrap(),
        latitude: 0.0,
        longitude: 0.0,
    };

    assert_eq!(point.temporal_distance(1500.0, 2000.0), 500.0);
    assert_eq!(point.temporal_distance(TIME0.timestamp() as f64, 50.0), 50.0);
}
