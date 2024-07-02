pub mod fileutils;
pub mod errors;

use leptos::*;
use crate::errors::Error;

enum SupportedInportTypes {
    GoogleRecordsJson,
    GPSTrackGPX,
}

pub type ErrorMessages = Vec<Error>;
pub fn log_error(error: Error) {
    use_context::<WriteSignal<ErrorMessages>>().expect("Unable to find contextual signal for ErrorMessages").update(|messages| messages.push(error));
}

pub fn set_processing(processing: bool) {
    use_context::<WriteSignal<bool>>().expect("Unable to find contextual signal for Processing Flag").set(processing);
}

pub fn end_processing(reason: Error) {
    set_processing(false);
    log_error(reason);
}