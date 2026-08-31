use super::*;

#[test]
fn test_parse_timestamp_str() {
    let timestamp = TimestampRfc3339("2015-01-25T09:11:16.547-08:00".to_string());
    let dt = parse_timestamp_str(&timestamp);
    assert!(dt.is_ok());
    assert_eq!(dt.unwrap().format("%Y-%m-%dT%H:%M:%S%.3f%:z").to_string(), "2015-01-25T17:11:16.547+00:00");
}

#[test]
fn test_parse_geolocation_e7() {
    let geolocation_e7 = GeoLocationE7(374219999);
    let geolocation = parse_geolocation_e7(&geolocation_e7);
    assert!(geolocation.is_ok());
    assert_eq!(geolocation.unwrap(), 37.4219999);
}

#[test]
fn test_parse_geolocation() {
    let geolocation = GeoLocation("geo:37.4219999,-122.0840576".to_string());
    let geolocation = JsonEntry::parse_geolocation(&geolocation);
    assert!(geolocation.is_ok());
    assert_eq!(geolocation.unwrap(), (37.4219999, -122.0840576));
}
