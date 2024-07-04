use web_sys::{File, FileReader};
use super::errors::FileProcessingError;
use wasm_bindgen::prelude::*;
use leptos::*;

/// Get filename from path
pub fn get_filename(path: &str) -> Result<String, FileProcessingError> {
    if !(path.is_empty() || path.ends_with('/') || path.ends_with('\\')) {
        match path.replace('\\', "/").split('/').last() {
            Some(name) => return Ok(name.to_string()),
            None => ()
        }
    }
    Err(FileProcessingError::InvalidPathError(path.to_string()))
}

#[derive(Clone)]
pub struct FileContent {
    pub filename: String,
    pub content: String
}
pub type FileResult = Result<FileContent, FileProcessingError>;

pub fn extract_file_data(file1_ref: &NodeRef<html::Input>, file2_ref: &NodeRef<html::Input>,
                    file1_setter: WriteSignal<FileResult>, file2_setter: WriteSignal<FileResult>) -> Result<(),  FileProcessingError> {
    let file1_input = file1_ref.get().unwrap();
    let file2_input = file2_ref.get().unwrap();

    if let (Some(file1), Some(file2)) = (
        file1_input.files().and_then(|list| list.get(0)),
        file2_input.files().and_then(|list| list.get(0)),
    ) {
        // Read and parse the files
        let filename1 = get_filename(&file1_input.value())?;
        let filename2 = get_filename(&file2_input.value())?;

        read_and_parse_file(filename1, file1, file1_setter)?;
        read_and_parse_file(filename2, file2, file2_setter)?;
        Ok(())
    } else {
        Err(FileProcessingError::MissingFileError)
    }
}

/// Read a file and return it's contents as a string
pub fn read_and_parse_file(filename: String, file: File, set_file_out: WriteSignal<FileResult>) -> Result<(), FileProcessingError> {
    let file_reader = match FileReader::new() {
        Ok(file_reader) => file_reader,
        Err(err) => return Err(FileProcessingError::FileReaderError(format!("{}: {:#?}", filename, err)))
    };
    let file_reader_clone: FileReader = file_reader.clone();
    let onloadend_callback: Closure<dyn Fn()> = Closure::wrap(Box::new(move || {
        match file_reader_clone.ready_state() {
            FileReader::DONE => match file_reader_clone.result() {
                Ok(js_value) => {
                    match js_value.as_string() {
                        Some(value_as_string) => {
                            match value_as_string.is_empty() {
                                true => set_file_out.set(Err(FileProcessingError::FileReaderError(format!("{}: is empty file", filename)))),
                                false => set_file_out.set(Ok(FileContent { filename: filename.clone(), content: value_as_string })) // Clone filename as it has been moved here but we'll need to refer to it later
                            }
                        }
                        None => set_file_out.set(Err(FileProcessingError::FileReaderError(format!("{}: can not be parsed as string", filename))))
                    }
                }
                Err(_) => set_file_out.set(Err(FileProcessingError::FileReaderError(format!("{}: Filereader unable to read as text", filename))))
            }
            _ => set_file_out.set(Err(FileProcessingError::FileReaderError(
                                format!("{}: Filereader State returned {}",
                                filename,
                                file_reader_clone.ready_state()))))
        }
    }) as Box<dyn Fn()>);
    file_reader.set_onloadend(Some(onloadend_callback.as_ref().unchecked_ref()));
    let _ = file_reader.read_as_text(&file);
    onloadend_callback.forget();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_filename() {
        assert_eq!(get_filename(r"C:\fakepath\location-history.json"), Ok("location-history.json".to_string()));
        assert_eq!(get_filename("/fakepath/location-history.json"), Ok("location-history.json".to_string()));
        assert_eq!(get_filename(""),  Err(FileProcessingError::InvalidPathError("".to_string())));
        assert_eq!(get_filename("/fakepath/"), Err(FileProcessingError::InvalidPathError("/fakepath/".to_string())));
        assert_eq!(get_filename(r"C:\fakepath\"), Err(FileProcessingError::InvalidPathError(r"C:\fakepath\".to_string())));
    }
}