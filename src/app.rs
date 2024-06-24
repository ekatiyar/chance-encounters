use leptos::*;
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    // Signal to track if both files are provided
    let (missing_files, set_missing_files) = create_signal(false);

    view! {
        <Home files_missing=set_missing_files/>
        <Show when=move || { missing_files.get() }>
            <MissingFilesAlert />
        </Show>
    }
}

#[component]
fn Home(files_missing: WriteSignal<bool>) -> impl IntoView {
    // References to the file input elements
    let file1_ref = create_node_ref::<html::Input>();
    let file2_ref = create_node_ref::<html::Input>();
    // Function to handle the Analyze button click
    let analyze = move |_| {
        let file1_input = file1_ref.get().unwrap();
        let file2_input = file2_ref.get().unwrap();

        if let (Some(file1), Some(file2)) = (
            file1_input.files().and_then(|list| list.get(0)),
            file2_input.files().and_then(|list| list.get(0)),
        ) {
            // Read and parse the files
            read_and_parse_file(file1);
            read_and_parse_file(file2);
            files_missing.set(false)
        } else {
            files_missing.set(true);
        }
    };
    view! {
        <div class="container mx-auto p-4 flex flex-col items-center justify-center min-h-screen">
            <h1 class="text-2xl font-bold mb-4">
                "Location History Analyzer"
            </h1>
            <p class="mb-4">
                "Upload two Google location history files to find the closest spatial and temporal points."
            </p>
            <div class="flex space-x-4 mb-4">
                <div class="form-control w-full max-w-xs">
                    <label class="label">
                        <span class="label-text">"Choose file 1"</span>
                    </label>
                    <input type="file" class="file-input file-input-bordered w-full max-w-xs" node_ref={file1_ref} />
                </div>
                <div class="form-control w-full max-w-xs">
                    <label class="label">
                        <span class="label-text">"Choose file 2"</span>
                    </label>
                    <input type="file" class="file-input file-input-bordered w-full max-w-xs" node_ref={file2_ref} />
                </div>
            </div>
            <button class="btn btn-primary" on:click={analyze}> "Analyze" </button>
        </div>
    }
}

#[component]
fn MissingFilesAlert() -> impl IntoView {
    view! {
        <div class="toast toast-top toast-end">
            <div class="alert alert-error alert-sm shadow-lg">
                <div>
                    <svg xmlns="(link unavailable)" fill="none" viewBox="0 0 24 24" class="stroke-current inline-block w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                    </svg>
                    <span>"Please provide both files."</span>
                </div>
            </div>
        </div>
    }
}

// Function to read and parse a file
fn read_and_parse_file(file: web_sys::File) {
    let file_reader = web_sys::FileReader::new().unwrap();
    let fr_c = file_reader.clone();

    let onloadend_cb = Closure::wrap(Box::new(move || {
        if fr_c.ready_state() == web_sys::FileReader::DONE {
            let result = fr_c.result().unwrap();
            let json_str = result.as_string().unwrap();
            let json_value: Value = serde_json::from_str(&json_str).unwrap();
            console_log(&format!("{:?}", json_value));
        }
    }) as Box<dyn Fn()>);

    file_reader.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
    file_reader.read_as_text(&file).unwrap();
    onloadend_cb.forget();
}

#[wasm_bindgen]
pub fn console_log(s: &str) {
    web_sys::console::log_1(&s.into());
}