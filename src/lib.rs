mod compute;
mod decoders;
mod model;

use compute::{cluster, find_candidates, find_candidates_weighted, AnalysisOptions, SearchMode};
use decoders::FileFormat;
use model::{SpaceTimePoint, SpaceTimeRecord};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Options coming from JS. Every field is optional; missing fields fall back to
/// [`AnalysisOptions::default`].
#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct OptionsInput {
    /// `"window"` (default) or `"speed"`.
    mode: Option<String>,
    top_n: Option<usize>,
    time_window_secs: Option<i64>,
    spatial_cap_km: Option<f64>,
    /// Assumed travel speed in km/h (speed mode); converted to km/s internally.
    speed_kmh: Option<f64>,
    space_eps_km: Option<f64>,
    time_eps_secs: Option<i64>,
}

impl OptionsInput {
    fn into_options(self) -> AnalysisOptions {
        let d = AnalysisOptions::default();
        let mode = match self.mode.as_deref() {
            Some("speed") => SearchMode::Speed,
            _ => SearchMode::Window,
        };
        AnalysisOptions {
            mode,
            top_n: self.top_n.unwrap_or(d.top_n),
            time_window_secs: self.time_window_secs.unwrap_or(d.time_window_secs),
            spatial_cap_km: self.spatial_cap_km.unwrap_or(d.spatial_cap_km),
            speed_km_s: self.speed_kmh.map(|kmh| kmh / 3600.0).unwrap_or(d.speed_km_s),
            space_eps_km: self.space_eps_km.unwrap_or(d.space_eps_km),
            time_eps_secs: self.time_eps_secs.unwrap_or(d.time_eps_secs),
        }
    }
}

#[derive(Serialize)]
struct PointOut {
    lat: f64,
    lon: f64,
    /// Epoch milliseconds (directly usable with `new Date(ms)`).
    start: i64,
    end: i64,
}

impl From<&SpaceTimePoint> for PointOut {
    fn from(p: &SpaceTimePoint) -> Self {
        PointOut {
            lat: p.latitude,
            lon: p.longitude,
            start: p.start_time.timestamp_millis(),
            end: p.end_time.timestamp_millis(),
        }
    }
}

#[derive(Serialize)]
struct EncounterOut {
    rank: usize,
    distance_km: f64,
    time_gap_secs: f64,
    cluster_size: usize,
    a: PointOut,
    b: PointOut,
}

#[derive(Serialize)]
struct Stats {
    points_a: usize,
    points_b: usize,
    clusters: usize,
}

#[derive(Serialize)]
struct AnalysisResult {
    encounters: Vec<EncounterOut>,
    stats: Stats,
}

/// Installs a readable panic message in the browser console. Safe to call from
/// the worker before doing any work.
#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}

fn report(progress: &js_sys::Function, pct: f64) {
    // Progress is best-effort; a throwing callback must not abort the analysis.
    let _ = progress.call1(&JsValue::NULL, &JsValue::from_f64(pct));
}

/// Parse two Google Takeout location histories and return the closest
/// space-time encounters between them.
///
/// `opts` is a plain JS object (see [`OptionsInput`]); `progress` is called with
/// a percentage in `[0, 100]` at phase boundaries.
#[wasm_bindgen]
pub fn analyze(
    file1: &str,
    file2: &str,
    opts: JsValue,
    progress: &js_sys::Function,
) -> Result<JsValue, JsValue> {
    let options = if opts.is_undefined() || opts.is_null() {
        AnalysisOptions::default()
    } else {
        serde_wasm_bindgen::from_value::<OptionsInput>(opts)
            .map_err(|e| JsValue::from_str(&format!("Invalid options: {e}")))?
            .into_options()
    };

    report(progress, 0.0);

    let mut record_a = SpaceTimeRecord::new(file1, FileFormat::Json)
        .map_err(|e| JsValue::from_str(&format!("File 1: {e}")))?;
    report(progress, 30.0);

    let mut record_b = SpaceTimeRecord::new(file2, FileFormat::Json)
        .map_err(|e| JsValue::from_str(&format!("File 2: {e}")))?;
    report(progress, 55.0);

    // Release builds skip the decoder's sorted debug_assert, so sort explicitly.
    record_a.points.sort_by_key(|p| p.start_time);
    record_b.points.sort_by_key(|p| p.start_time);
    report(progress, 65.0);

    let candidates = match options.mode {
        SearchMode::Window => find_candidates(&record_a.points, &record_b.points, &options),
        SearchMode::Speed => find_candidates_weighted(&record_a.points, &record_b.points, &options),
    };
    report(progress, 90.0);

    let encounters = cluster(candidates, &record_a.points, &options);

    let out = AnalysisResult {
        stats: Stats {
            points_a: record_a.points.len(),
            points_b: record_b.points.len(),
            clusters: encounters.len(),
        },
        encounters: encounters
            .iter()
            .enumerate()
            .map(|(i, e)| EncounterOut {
                rank: i + 1,
                distance_km: e.distance_km,
                time_gap_secs: e.time_gap_secs,
                cluster_size: e.cluster_size,
                a: (&record_a.points[e.a_index]).into(),
                b: (&record_b.points[e.b_index]).into(),
            })
            .collect(),
    };

    report(progress, 100.0);

    serde_wasm_bindgen::to_value(&out)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {e}")))
}
