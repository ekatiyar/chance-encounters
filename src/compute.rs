use crate::model::{SpaceTimePoint, EARTH_RADIUS_KM};
use rstar::{RTree, RTreeObject, AABB, PointDistance};

/// Which matching strategy the search uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchMode {
    /// Hard gates on space and time (`spatial_cap_km` / `time_window_secs`).
    Window,
    /// No hard cutoff: space and time are blended via an assumed travel speed
    /// and every shorter-side point keeps its single nearest counterpart.
    Speed,
}

/// Tunable knobs for the encounter search.
#[derive(Clone, Copy, Debug)]
pub struct AnalysisOptions {
    /// Matching strategy (window vs. speed-weighted).
    pub mode: SearchMode,
    /// Maximum number of clustered encounters to return.
    pub top_n: usize,
    /// Temporal gate half-width in seconds: a B-point is considered against an
    /// A-point only if their intervals fall within +/- this of each other.
    /// Window mode only.
    pub time_window_secs: i64,
    /// Discard candidate pairs farther apart than this (km). Window mode only.
    pub spatial_cap_km: f64,
    /// Speed-mode exchange rate (km per second) between the spatial and temporal
    /// axes: a gap of `t` seconds counts as `speed_km_s * t` km of separation.
    pub speed_km_s: f64,
    /// Two candidates within this spatial distance (km) of a cluster
    /// representative are treated as the same meetup.
    pub space_eps_km: f64,
    /// Two candidates within this temporal gap (secs) of a cluster
    /// representative are treated as the same meetup.
    pub time_eps_secs: i64,
}

impl Default for AnalysisOptions {
    fn default() -> Self {
        Self {
            mode: SearchMode::Window,
            top_n: 20,
            time_window_secs: 86_400,
            spatial_cap_km: 100.0,
            // Driving: 100 km/h expressed as km/s.
            speed_km_s: 100.0 / 3600.0,
            space_eps_km: 0.5,
            time_eps_secs: 3600,
        }
    }
}

/// A raw A/B pair kept by a candidate finder.
#[derive(Clone, Copy, Debug)]
pub struct Candidate {
    pub a_index: usize,
    pub b_index: usize,
    /// Real great-circle distance (km), for display.
    pub distance_km: f64,
    /// Real temporal gap (secs), for display.
    pub time_gap_secs: f64,
    /// Value used to rank/cluster candidates closest-first. In window mode this
    /// is the spatial distance; in speed mode it is the space/time blend.
    pub rank_km: f64,
}

/// A distinct clustered encounter: the closest pair of its cluster.
#[derive(Clone, Copy, Debug)]
pub struct Encounter {
    pub a_index: usize,
    pub b_index: usize,
    pub distance_km: f64,
    pub time_gap_secs: f64,
    pub cluster_size: usize,
}

/// Time-gated two-pointer sweep over two time-sorted, non-overlapping point
/// lists. For each A-point, the active B-window `[lo, hi)` holds exactly the
/// B-points whose interval is within `time_window_secs` of the A-point; each is
/// scored spatially and kept if under `spatial_cap_km`.
pub fn find_candidates(a: &[SpaceTimePoint], b: &[SpaceTimePoint], opts: &AnalysisOptions) -> Vec<Candidate> {
    let window = opts.time_window_secs;
    let mut candidates = Vec::new();
    let mut lo = 0usize;
    let mut hi = 0usize;

    for (a_index, ap) in a.iter().enumerate() {
        let a_start = ap.start_time.timestamp();
        let a_end = ap.end_time.timestamp();

        // Pull in B-points that begin on or before the A-window's upper edge.
        while hi < b.len() && b[hi].start_time.timestamp() <= a_end + window {
            hi += 1;
        }
        // Drop B-points that ended before the A-window's lower edge.
        while lo < hi && b[lo].end_time.timestamp() < a_start - window {
            lo += 1;
        }

        for (offset, bp) in b[lo..hi].iter().enumerate() {
            let distance_km = ap.haversine_distance(bp.latitude, bp.longitude);
            if distance_km > opts.spatial_cap_km {
                continue;
            }
            let time_gap_secs = ap.temporal_distance(
                bp.start_time.timestamp() as f64,
                bp.end_time.timestamp() as f64,
            );
            candidates.push(Candidate {
                a_index,
                b_index: lo + offset,
                distance_km,
                time_gap_secs,
                rank_km: distance_km,
            });
        }
    }

    candidates
}

/// Speed-weighted candidate finder. Blends space and time into a single distance
/// using `opts.speed_km_s` as the exchange rate and, for **every** point on the
/// shorter side, keeps its single nearest counterpart on the longer side. There
/// is no cutoff, so the result has exactly one candidate per shorter-side point
/// (and is non-empty whenever both inputs are non-empty).
pub fn find_candidates_weighted(
    a: &[SpaceTimePoint],
    b: &[SpaceTimePoint],
    opts: &AnalysisOptions,
) -> Vec<Candidate> {
    let speed = opts.speed_km_s;
    // Index the longer side; probe with the shorter side so the tree query count
    // is minimized and every shorter-side point yields a match.
    let a_is_longer = a.len() >= b.len();
    let (long, short) = if a_is_longer { (a, b) } else { (b, a) };

    // One projection shared by the tree and the query points so rstar's Euclidean
    // envelope metric matches the object metric exactly (see `Projection`).
    let projection = Projection::for_points(a, b);

    let tree = RTree::bulk_load(
        long.iter()
            .enumerate()
            .map(|(i, p)| WeightedPoint::new(p, i, speed, &projection))
            .collect(),
    );

    let mut candidates = Vec::with_capacity(short.len());
    for (short_index, sp) in short.iter().enumerate() {
        let query = projection.query_point(sp, speed);
        let Some(nn) = tree.nearest_neighbor(&query) else {
            continue;
        };
        let (a_index, b_index) = if a_is_longer {
            (nn.index, short_index)
        } else {
            (short_index, nn.index)
        };

        let ap = &a[a_index];
        let bp = &b[b_index];
        let distance_km = ap.haversine_distance(bp.latitude, bp.longitude);
        let time_gap_secs = ap.temporal_distance(
            bp.start_time.timestamp() as f64,
            bp.end_time.timestamp() as f64,
        );
        // Blended metric with the temporal gap converted to km via the speed.
        let temporal_km = time_gap_secs * speed;
        let rank_km = (distance_km * distance_km + temporal_km * temporal_km).sqrt();

        candidates.push(Candidate {
            a_index,
            b_index,
            distance_km,
            time_gap_secs,
            rank_km,
        });
    }

    candidates
}

/// Cluster candidates into distinct events. Sorted by distance ascending, each
/// candidate is absorbed by the first existing representative it is within
/// `(space_eps_km, time_eps_secs)` of (compared on the A-point); otherwise it
/// starts a new cluster. Because we process closest-first, the representative is
/// the closest pair of its cluster. Returns the top-N representatives.
pub fn cluster(mut candidates: Vec<Candidate>, a: &[SpaceTimePoint], opts: &AnalysisOptions) -> Vec<Encounter> {
    candidates.sort_by(|x, y| {
        x.rank_km
            .partial_cmp(&y.rank_km)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut reps: Vec<Encounter> = Vec::new();
    'candidates: for cand in candidates {
        let ap = &a[cand.a_index];
        for rep in reps.iter_mut() {
            let rep_ap = &a[rep.a_index];
            let space = ap.haversine_distance(rep_ap.latitude, rep_ap.longitude);
            let time = ap.temporal_distance(
                rep_ap.start_time.timestamp() as f64,
                rep_ap.end_time.timestamp() as f64,
            );
            if space <= opts.space_eps_km && time <= opts.time_eps_secs as f64 {
                rep.cluster_size += 1;
                continue 'candidates;
            }
        }
        reps.push(Encounter {
            a_index: cand.a_index,
            b_index: cand.b_index,
            distance_km: cand.distance_km,
            time_gap_secs: cand.time_gap_secs,
            cluster_size: 1,
        });
    }

    reps.truncate(opts.top_n);
    reps
}

/// Equirectangular projection of lat/lon into km around a fixed mean latitude, so
/// that plain Euclidean distance in the projected plane approximates great-circle
/// distance. Sharing one projection across the tree and the query points lets
/// rstar's Euclidean envelope metric match the object `distance_2` exactly, which
/// the rstar contract requires for correct nearest-neighbor results.
struct Projection {
    cos_lat0: f64,
}

impl Projection {
    /// Build a projection whose reference latitude is the mean latitude over the
    /// combined points of both inputs, so the tree and queries share one frame.
    fn for_points(a: &[SpaceTimePoint], b: &[SpaceTimePoint]) -> Self {
        let mut sum = 0.0;
        let mut count = 0.0;
        for p in a.iter().chain(b.iter()) {
            sum += p.latitude;
            count += 1.0;
        }
        let mean_lat = if count > 0.0 { sum / count } else { 0.0 };
        Projection {
            cos_lat0: mean_lat.to_radians().cos(),
        }
    }

    fn x_km(&self, longitude: f64) -> f64 {
        EARTH_RADIUS_KM * longitude.to_radians() * self.cos_lat0
    }

    fn y_km(&self, latitude: f64) -> f64 {
        EARTH_RADIUS_KM * latitude.to_radians()
    }

    /// Query point for a shorter-side point: its projected position plus its
    /// midpoint time scaled into km, as a degenerate `[x, y, t]` probe.
    fn query_point(&self, p: &SpaceTimePoint, speed: f64) -> [f64; 3] {
        let mid = (p.start_time.timestamp() as f64 + p.end_time.timestamp() as f64) / 2.0;
        [self.x_km(p.longitude), self.y_km(p.latitude), mid * speed]
    }
}

/// Point-to-interval distance: `0.0` when `t` lies inside `[lo, hi]`, otherwise
/// the distance to the nearer endpoint. Exactly equals the 1-D component of
/// `AABB::distance_2` for an extent envelope on the time axis.
fn clamp_gap(lo: f64, hi: f64, t: f64) -> f64 {
    (lo - t).max(0.0).max(t - hi)
}

/// An rstar-indexable point in the projected space/time coordinate space. Built
/// from a [`SpaceTimePoint`] via a shared [`Projection`] plus the search speed;
/// carries the source `index` so the nearest-neighbor result can be mapped back
/// to the original slice. The spatial dims are a single point; the time dim is an
/// extent `[t0_km, t1_km]` so overlapping intervals score a zero gap.
struct WeightedPoint {
    x_km: f64,
    y_km: f64,
    t0_km: f64,
    t1_km: f64,
    index: usize,
}

impl WeightedPoint {
    fn new(p: &SpaceTimePoint, index: usize, speed: f64, projection: &Projection) -> Self {
        WeightedPoint {
            x_km: projection.x_km(p.longitude),
            y_km: projection.y_km(p.latitude),
            t0_km: p.start_time.timestamp() as f64 * speed,
            t1_km: p.end_time.timestamp() as f64 * speed,
            index,
        }
    }
}

impl RTreeObject for WeightedPoint {
    type Envelope = AABB<[f64; 3]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(
            [self.x_km, self.y_km, self.t0_km],
            [self.x_km, self.y_km, self.t1_km],
        )
    }
}

impl PointDistance for WeightedPoint {
    fn distance_2(&self, point: &[f64; 3]) -> f64 {
        let dx = self.x_km - point[0];
        let dy = self.y_km - point[1];
        let dt = clamp_gap(self.t0_km, self.t1_km, point[2]);
        dx * dx + dy * dy + dt * dt
    }
}

#[cfg(test)]
mod analysis_tests;

