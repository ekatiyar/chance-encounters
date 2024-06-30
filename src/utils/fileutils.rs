use std::path::Path;
use web_sys::{File, FileReader};
use wasm_bindgen::prelude::*;
use leptos::*;

// Function to get filename from path
pub fn get_filename(path: &str) -> Option<String> {
    if path.is_empty() || path.ends_with('/') || path.ends_with('\\') {
        None
    } else {
        Path::new(path)
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .map(|s| s.replace('\\', "/"))
            .map(|s| s.split('/').last().unwrap().to_string())
    }
}

// Struct to hold filename and web_sys::File object
pub struct FileDesc {
    pub filename: String,
    pub file: File
}
pub type FileResult = Result<String, String>;

// Function to read and parse a file to json
pub fn read_and_parse_file(file_desc: FileDesc, set_file_out: WriteSignal<FileResult>) {
    let (filename, file) = (file_desc.filename, file_desc.file);
    let file_reader = FileReader::new().unwrap();
    let file_reader_clone: FileReader = file_reader.clone();
    let onloadend_callback: Closure<dyn Fn()> = Closure::wrap(Box::new(move || {
        match file_reader_clone.ready_state() {
            FileReader::DONE => match file_reader_clone.result() {
                Ok(js_value) => {
                    match js_value.as_string() {
                        Some(value_as_string) => {
                            match value_as_string.is_empty() {
                                true => set_file_out.set(Err(format!("{}: is empty file", filename))),
                                false => set_file_out.set(Ok(value_as_string))
                            }
                        }
                        None => set_file_out.set(Err(format!("{}: can not be parsed as string", filename)))
                    }
                }
                Err(_) => set_file_out.set(Err(format!("{}: Filereader unable to read as text", filename)))
            }
            _ => set_file_out.set(Err(format!("{}: Filereader not ready. State returned {}",
                        filename,
                        file_reader_clone.ready_state())))
        }
    }) as Box<dyn Fn()>);
    file_reader.set_onloadend(Some(onloadend_callback.as_ref().unchecked_ref()));
    let _ = file_reader.read_as_text(&file);
    onloadend_callback.forget();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_filename() {
        assert_eq!(get_filename(r"C:\fakepath\location-history.json"), Some("location-history.json".to_string()));
        assert_eq!(get_filename("/fakepath/location-history.json"), Some("location-history.json".to_string()));
        assert_eq!(get_filename(""), None);
        assert_eq!(get_filename("/fakepath/"), None);
        assert_eq!(get_filename(r"C:\fakepath\"), None);
    }
}