use leptos::*;
use crate::utils::{fileutils::*, utils::*};
use std::cmp::Ordering;

#[component]
pub fn App() -> impl IntoView {
    // Signal to track if both files are provided
    let (error_messages, set_error_messages) = create_signal::<ErrorMessages>(ErrorMessages::new());
    let (processing_files, set_processing_files) = create_signal(false);
    provide_context(set_error_messages);
    provide_context(set_processing_files);

    view! {
        <Home button_clicked=processing_files/>
        <ErrorAlerts error_messages/>
    }
}

#[component]
fn Home(button_clicked: ReadSignal<bool>) -> impl IntoView {
    // References to the file input elements
    let file1_ref = create_node_ref::<html::Input>();
    let file2_ref = create_node_ref::<html::Input>();

    let (file1_result, set_file1_result) = create_signal::<FileResult>(Err(String::new()));
    let (file2_result, set_file2_result) = create_signal::<FileResult>(Err(String::new()));
    
    // Function to handle the Analyze button click
    let analyze = move |_| {
        // Reset the results
        set_file1_result.set_untracked(Err(String::new()));
        set_file2_result.set_untracked(Err(String::new()));
        use_context::<WriteSignal<Vec<String>>>().unwrap().update(|messages| messages.clear());
        
        let populated_files = extract_file_data(&file1_ref, &file2_ref, set_file1_result, set_file2_result);
        if populated_files {
            set_processing(true);
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
            <Show when=move || button_clicked.get()>
                <ResultDisplay file1_result=file1_result file2_result=file2_result/>
            </Show>
        </div>
    }
}

fn extract_file_data(
    file1_ref: &NodeRef<html::Input>,
    file2_ref: &NodeRef<html::Input>,
    file1_setter: WriteSignal<FileResult>,
    file2_setter: WriteSignal<FileResult>) -> bool{
    let file1_input = file1_ref.get().unwrap();
        let file2_input = file2_ref.get().unwrap();

        if let (Some(file1), Some(file2)) = (
            file1_input.files().and_then(|list| list.get(0)),
            file2_input.files().and_then(|list| list.get(0)),
        ) {
            // Read and parse the files
            let filename1 = get_filename(&file1_ref.get().unwrap().value()).unwrap();
            let filename2 = get_filename(&file2_ref.get().unwrap().value()).unwrap();

            read_and_parse_file(FileDesc{filename: filename1, file: file1}, file1_setter);
            read_and_parse_file(FileDesc{filename: filename2, file: file2}, file2_setter);
            return true
        } else {
            log_error("Please provide both files.".to_string());
        }
        false
}

#[component]
fn LoadingSpinner() -> impl IntoView {
    view! {
        <div class="flex justify-center items-center">
            <span class="loading loading-dots loading-lg"></span>
        </div>
    }
}

fn process_data(file1: String, file2: String) -> String
{
    let mut diff = String::new();
    let lines1 = file1.lines().collect::<Vec<&str>>();
    let lines2 = file2.lines().collect::<Vec<&str>>();
    let min_len = lines1.len().min(lines2.len());
    logging::log!("min_len: {}", min_len);
    for i in 0..min_len {
        if lines1[i] != lines2[i] {
            diff.push_str(&format!("- {}\n+ {}\n", lines1[i], lines2[i]));
        } else {
            diff.push_str(&format!("  {}\n", lines1[i]));
        }
    }
    match lines1.len().cmp(&lines2.len()) {
        Ordering::Greater => diff.push_str(&format!("- {}\n", lines1[min_len..].join("\n"))),
        Ordering::Less => diff.push_str(&format!("+ {}\n", lines2[min_len..].join("\n"))),
        Ordering::Equal => (),
    }
    
    diff
    // TODO: Implement actual analysis logic here
}

#[component]
fn ResultDisplay(file1_result: ReadSignal<FileResult>, file2_result: ReadSignal<FileResult>) -> impl IntoView {
    let derived_signal = create_memo(move |_| file1_result.get().is_ok() && file2_result.get().is_ok());
    view! {
        <Show when=move || derived_signal.get() fallback=move || {
            logging::log!("loading...");
            match file1_result.get() {
                Ok(_) => (),
                Err(error) => {
                    if !error.is_empty() {
                        set_processing(false);
                        log_error(error);
                    }
                }
            };
            match file2_result.get() {
                Ok(_) => (),
                Err(error) => {
                    if !error.is_empty() {
                        set_processing(false);
                        log_error(error);
                    }
                }
            };
            view! { <LoadingSpinner /> } 
        }>
            <div class="mt-4 w-full">
                <h2 class="text-xl font-bold mb-2">"Analysis Results"</h2>
                    <h3 class="text-lg font-semibold mb-2">"File 1 Data"</h3>
                    <div class="mockup-code h-64 overflow-auto">
                        <pre><code>{process_data(file1_result.get().unwrap(), file2_result.get().unwrap())}</code></pre>
                    </div>
            </div>
        </Show>
    }
}

#[component]
fn ErrorAlerts(error_messages: ReadSignal<Vec<String>>) -> impl IntoView {
    view! {
        <div class="toast toast-top toast-end">
        {
            move || error_messages.get().into_iter().map(|error_message| view! {
                <div class="alert alert-error alert-sm shadow-lg">
                    <div>
                        <svg xmlns="(link unavailable)" fill="none" viewBox="0 0 24 24" class="stroke-current inline-block w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                        </svg>
                        <span>{error_message}</span>
                    </div>
                </div>
            }).collect_view()
        }
        </div>
    }
}
