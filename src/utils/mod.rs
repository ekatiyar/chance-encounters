pub mod fileutils;

use leptos::*;

enum SupportedInportTypes {
    GoogleRecordsJson,
    GPSTrackGPX,
}

pub type ErrorMessages = Vec<String>;
pub fn log_error(error: String) {
    use_context::<WriteSignal<ErrorMessages>>().unwrap().update(|messages| messages.push(error));
}

pub fn set_processing(processing: bool) {
    use_context::<WriteSignal<bool>>().unwrap().set(processing);
}

pub fn end_processing(reason: String) {
    set_processing(false);
    log_error(reason);
}