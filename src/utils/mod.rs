pub mod fileutils;
pub mod errors;

use leptos::*;
use shrinkwraprs::Shrinkwrap;
use crate::errors::Error;

enum SupportedInportTypes {
    GoogleRecordsJson,
    GPSTrackGPX,
}

#[derive(Shrinkwrap, Clone)]
#[shrinkwrap(mutable)]
pub struct ErrorMessages(pub Vec<Error>);
pub fn log_error(error: Error) {
    use_context::<WriteSignal<ErrorMessages>>().expect("Unable to find contextual signal for ErrorMessages").update(|messages| messages.push(error));
}

pub fn clear_error_messages() {
    use_context::<WriteSignal<ErrorMessages>>().expect("Unable to find contextual signal for ErrorMessages").update(|messages| messages.clear());
}

pub fn set_processing(processing: bool) {
    use_context::<WriteSignal<bool>>().expect("Unable to find contextual signal for Processing Flag").set(processing);
}

pub fn end_processing(reason: Error) {
    set_processing(false);
    log_error(reason);
}