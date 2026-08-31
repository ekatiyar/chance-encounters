use super::*;
use crate::model::SpaceTimePoint;
use chrono::DateTime;

fn pt(lat: f64, lon: f64, start: i64, end: i64) -> SpaceTimePoint {
    SpaceTimePoint {
        latitude: lat,
        longitude: lon,
        start_time: DateTime::from_timestamp(start, 0).unwrap(),
        end_time: DateTime::from_timestamp(end, 0).unwrap(),
    }
}

fn opts() -> AnalysisOptions {
    AnalysisOptions {
        mode: SearchMode::Window,
        top_n: 10,
        time_window_secs: 3600,
        spatial_cap_km: 5.0,
        speed_km_s: 100.0 / 3600.0,
        space_eps_km: 1.0,
        time_eps_secs: 3600,
    }
}

fn speed_opts(speed_km_s: f64) -> AnalysisOptions {
    AnalysisOptions {
        mode: SearchMode::Speed,
        speed_km_s,
        ..opts()
    }
}

#[test]
fn finds_known_closest_pair() {
    let a = vec![
        pt(0.0, 0.0, 0, 100),
        pt(10.0, 10.0, 200, 300),
        pt(50.0, 50.0, 1000, 1100),
    ];
    let b = vec![
        pt(20.0, 20.0, 0, 100),
        pt(10.001, 10.001, 210, 290),
        pt(80.0, 80.0, 1000, 1100),
    ];

    let candidates = find_candidates(&a, &b, &opts());
    let encounters = cluster(candidates, &a, &opts());

    assert!(!encounters.is_empty(), "expected at least one encounter");
    // The a[1]/b[1] pair is the spatially closest, so it ranks first.
    assert_eq!(encounters[0].a_index, 1);
    assert_eq!(encounters[0].b_index, 1);
    assert!(encounters[0].distance_km < 1.0, "distance was {}", encounters[0].distance_km);
}

#[test]
fn excludes_pairs_outside_time_window() {
    // Same place, but ~27 hours apart — well beyond the +/-1h window.
    let a = vec![pt(0.0, 0.0, 0, 100)];
    let b = vec![pt(0.0, 0.0, 100_000, 100_100)];

    let candidates = find_candidates(&a, &b, &opts());
    assert!(candidates.is_empty(), "expected no candidates, got {}", candidates.len());
}

#[test]
fn dedups_continuous_meetup_into_one_event() {
    // A single stationary B-point spanning the whole dwell.
    let b = vec![pt(5.0, 5.0, 0, 10_000)];
    // A lingers nearby across several consecutive points.
    let a = vec![
        pt(5.001, 5.001, 100, 200),
        pt(5.001, 5.0011, 200, 300),
        pt(5.0009, 5.001, 300, 400),
        pt(5.0011, 5.0009, 400, 500),
    ];

    let candidates = find_candidates(&a, &b, &opts());
    assert_eq!(candidates.len(), 4, "each A-point should match the B-point");

    let encounters = cluster(candidates, &a, &opts());
    assert_eq!(encounters.len(), 1, "the continuous dwell should collapse to one event");
    assert_eq!(encounters[0].cluster_size, 4);
}

/// Brute-force oracle: for each shorter-side point pick the longer-side point
/// minimizing the real blended metric, returning `(a_index, b_index)` per
/// candidate in short-side order.
fn brute_force_weighted(
    a: &[SpaceTimePoint],
    b: &[SpaceTimePoint],
    speed: f64,
) -> Vec<(usize, usize)> {
    let a_is_longer = a.len() >= b.len();
    let (long, short) = if a_is_longer { (a, b) } else { (b, a) };

    short
        .iter()
        .enumerate()
        .map(|(si, sp)| {
            let best = (0..long.len())
                .min_by(|&i, &j| {
                    let di = blended(sp, &long[i], speed);
                    let dj = blended(sp, &long[j], speed);
                    di.partial_cmp(&dj).unwrap()
                })
                .unwrap();
            if a_is_longer {
                (best, si)
            } else {
                (si, best)
            }
        })
        .collect()
}

fn blended(sp: &SpaceTimePoint, lp: &SpaceTimePoint, speed: f64) -> f64 {
    let dist = sp.haversine_distance(lp.latitude, lp.longitude);
    let gap = sp.temporal_distance(
        lp.start_time.timestamp() as f64,
        lp.end_time.timestamp() as f64,
    );
    let temporal_km = gap * speed;
    (dist * dist + temporal_km * temporal_km).sqrt()
}

#[test]
fn speed_mode_overlapping_interval_selects_true_nearest() {
    // Regression for the invalid-lower-bound bug: the old endpoint-delta
    // envelope over-estimated distance for overlapping intervals and pruned
    // the true nearest. Query (short side) sits at the origin with a wide
    // interval. Longer side:
    //   - a spatially-far point whose interval fully overlaps the query,
    //   - a spatially-near point whose interval is temporally offset.
    // At a modest speed the near/offset point is the true nearest; the old
    // tree would have pruned it. Assert we match brute force.
    let short = vec![pt(0.0, 0.0, 0, 100_000)];
    let long = vec![
        pt(1.0, 1.0, 0, 100_000),          // far, fully overlapping in time
        pt(0.001, 0.001, 100_200, 100_300), // near, just-offset in time
    ];
    // short is A (shorter); long is B.
    let speed = 0.001;
    let candidates = find_candidates_weighted(&short, &long, &speed_opts(speed));
    let oracle = brute_force_weighted(&short, &long, speed);
    assert_eq!(candidates.len(), oracle.len());
    for (cand, (a_idx, b_idx)) in candidates.iter().zip(oracle.iter()) {
        assert_eq!((cand.a_index, cand.b_index), (*a_idx, *b_idx));
    }
    // Sanity: the near/offset point (index 1) is the selected nearest.
    assert_eq!(candidates[0].b_index, 1);
}

#[test]
fn speed_mode_projection_preserves_regions() {
    // Two well-separated regions; each shorter-side point must map to its own
    // region's longer-side point. Guards the projection sign/scale.
    let short = vec![pt(0.0, 0.0, 0, 100), pt(50.0, 50.0, 0, 100)];
    let long = vec![
        pt(0.01, 0.01, 0, 100),
        pt(50.01, 50.01, 0, 100),
        pt(-40.0, -120.0, 0, 100),
    ];
    let speed = 0.01;
    let candidates = find_candidates_weighted(&short, &long, &speed_opts(speed));
    let oracle = brute_force_weighted(&short, &long, speed);
    for (cand, (a_idx, b_idx)) in candidates.iter().zip(oracle.iter()) {
        assert_eq!((cand.a_index, cand.b_index), (*a_idx, *b_idx));
    }
    // short is A; region 0 point -> long[0], region 1 point -> long[1].
    assert_eq!(candidates[0].b_index, 0);
    assert_eq!(candidates[1].b_index, 1);
}

#[test]
fn speed_mode_matches_every_shorter_side_point() {
    // A is the shorter side; every A-point is spatially far from every
    // B-point but temporally aligned. Window mode would return nothing;
    // speed mode must return one candidate per shorter-side point.
    let a = vec![pt(0.0, 0.0, 0, 100), pt(0.0, 0.0, 200, 300)];
    let b = vec![
        pt(40.0, 40.0, 0, 100),
        pt(41.0, 41.0, 200, 300),
        pt(42.0, 42.0, 400, 500),
    ];

    let candidates = find_candidates_weighted(&a, &b, &speed_opts(0.01));
    assert_eq!(candidates.len(), a.len(), "one candidate per shorter-side point");
    assert!(!candidates.is_empty());
}

#[test]
fn speed_mode_ranking_shifts_with_speed() {
    // Two isolated regions, so each A-point's nearest B is unambiguous.
    // Region 0: far apart (~111 km) but exactly the same time.
    // Region 1: nearly co-located (~11 m) but 1000 s apart.
    let a = vec![pt(0.0, 0.0, 0, 0), pt(50.0, 50.0, 0, 0)];
    let b = vec![pt(1.0, 0.0, 0, 0), pt(50.0001, 50.0, 1000, 1000)];

    // Low speed: time barely counts, so the near/different-time pair wins.
    let low = cluster(find_candidates_weighted(&a, &b, &speed_opts(0.001)), &a, &speed_opts(0.001));
    assert_eq!(low[0].a_index, 1, "low speed should rank the near pair first");

    // High speed: time dominates, so the far/same-time pair wins.
    let high = cluster(find_candidates_weighted(&a, &b, &speed_opts(1.0)), &a, &speed_opts(1.0));
    assert_eq!(high[0].a_index, 0, "high speed should rank the same-time pair first");
}

#[test]
fn speed_mode_clusters_continuous_dwell() {
    // A (shorter, 4 pts) lingers next to one of B's points; B's other points
    // are far away. Each A-point's nearest is the co-located B-point, and the
    // four near-identical A positions must collapse to a single event.
    let a = vec![
        pt(5.001, 5.001, 100, 200),
        pt(5.001, 5.0011, 200, 300),
        pt(5.0009, 5.001, 300, 400),
        pt(5.0011, 5.0009, 400, 500),
    ];
    let b = vec![
        pt(5.0, 5.0, 0, 10_000),
        pt(40.0, 40.0, 0, 10_000),
        pt(-30.0, -30.0, 0, 10_000),
        pt(10.0, -80.0, 0, 10_000),
        pt(-60.0, 20.0, 0, 10_000),
    ];

    let candidates = find_candidates_weighted(&a, &b, &speed_opts(0.01));
    assert_eq!(candidates.len(), 4, "each shorter-side (A) point yields a candidate");

    let encounters = cluster(candidates, &a, &speed_opts(0.01));
    assert_eq!(encounters.len(), 1, "the continuous dwell should collapse to one event");
    assert_eq!(encounters[0].cluster_size, 4);
}
