use leptos::*;
use leptos_workers::worker;
use crate::errors::Error;
use crate::decoders::*;
use crate::compute::{spacetime::SpaceTimeRecord, ChanceEncounter, get_nearest_points};
use crate::utils::{fileutils::*, *, errors::FileProcessingError};

#[component]
pub fn App() -> impl IntoView {
    // Signal to track if both files are provided
    let (error_messages, set_error_messages) = create_signal::<ErrorMessages>(ErrorMessages(vec![]));
    let (button_clicked, set_button_clicked) = create_signal(false);
    provide_context(set_error_messages);
    provide_context(set_button_clicked);

    view! {
        <Home button_clicked=button_clicked/>
        <ErrorAlerts error_messages/>
    }
}

#[component]
fn Home(button_clicked: ReadSignal<bool>) -> impl IntoView {
    // References to the file input elements
    let file1_ref = create_node_ref::<html::Input>();
    let file2_ref = create_node_ref::<html::Input>();

    let (file1_result, set_file1_result) = create_signal::<FileResult>(Err(FileProcessingError::InProcessError));
    let (file2_result, set_file2_result) = create_signal::<FileResult>(Err(FileProcessingError::InProcessError));
    let file_contents = create_memo(move |_| {
        logging::log!("Files Loaded: {}, {}", file1_result.get().is_ok(), file2_result.get().is_ok());
        match (file1_result.get(), file2_result.get()) {
            (Ok(file1), Ok(file2)) => Some((file1, file2)),
            _ => None
        }
    });

    // Load files when the button is clicked
    let load_files = move |_| {
        set_file1_result.set(Err(FileProcessingError::InProcessError));
        set_file2_result.set(Err(FileProcessingError::InProcessError));
        clear_error_messages();

        let (file_info1, file_info2) = match (get_file_info(&file1_ref), get_file_info(&file2_ref)) {
            (Ok(file1), Ok(file2)) => (file1, file2),
            (Err(err), _) | (_, Err(err)) => return end_processing(Error::from(err)),
        };
        match (process_file(file_info1, set_file1_result), process_file(file_info2, set_file2_result)) {
            (Err(err), _) | (_, Err(err)) => return end_processing(Error::from(err)),
            _ => (),
        };
        set_processing(true);
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
            <button class="btn btn-primary" on:click={load_files}> "Analyze" </button>
            <Show when=move || button_clicked.get()>
                <ResultDisplay file_contents/>
            </Show>
        </div>
    }
}

#[worker(MyFutureWorker)]
pub async fn process_data(files: FileContents) -> Result<String, Error>
{
    let (file1, file2) = match files {
        Some((file1, file2)) => (file1, file2),
        None => return Err(Error::from(FileProcessingError::MissingFileError))
    };
    logging::log!("Running WebWorker...");
    let record1 = SpaceTimeRecord::new(&file1.content, FileFormat::Json)?;
    let record2 = SpaceTimeRecord::new(&file2.content, FileFormat::Json)?;
    let closest_points = get_nearest_points(record1, record2);

    let num_points = closest_points.len();
    for point in &closest_points {
        logging::log!("Chance Encounter: {:?} and {:?} with a distance of {} km and {} s", point.point1, point.point2, point.distance_km, point.distance_s);
    }
    Ok(format!("Data Processed. Retrieved {} nearest points", closest_points.len()))
    // TODO: Implement actual analysis logic here
}

#[component]
fn ResultDisplay(file_contents: Memo<Option<(FileContent, FileContent)>>) -> impl IntoView {
    let response = create_local_resource(|| {}, move |_| {
        process_data(file_contents.get())
    });
    view! {
        {move || match response.get() {
            None => view! { <LoadingSpinner /> },
            Some(result) => {
                match result {
                    Ok(analysis_result) => match analysis_result {
                        Ok(analysis_result) => view! { <AnalysisResult analysis_result/> },
                        Err(error) => {
                            match error {
                                Error::FileProcessingError(FileProcessingError::MissingFileError) => {},
                                _ => end_processing(error)
                            }
                            // This won't be reached as ResultDisplay is hidden when end_processing is called
                            view! { <LoadingSpinner /> }
                        }
                    },
                    Err(error) => {
                        end_processing(Error::WebWorkerError(error.to_string()));
                        // This won't be reached as ResultDisplay is hidden when end_processing is called
                        view! { <LoadingSpinner /> }
                    }
                }
            }
        }}
    }
}

#[component]
fn AnalysisResult(analysis_result: String) -> impl IntoView {
    view! {
        <div class="mt-4 w-full">
        <h2 class="text-xl font-bold mb-2">"Analysis Results"</h2>
            <h3 class="text-lg font-semibold mb-2">"File 1 Data"</h3>
            <div class="mockup-code h-64 overflow-auto">
                <pre><code>{analysis_result}</code></pre>
            </div>
        </div>
    }
}

#[component]
fn LoadingSpinner() -> impl IntoView {
    view! {
        <div class="flex justify-center items-center">
            <span class="loading loading-dots loading-lg"></span>
        </div>
    }
}

#[component]
fn ErrorAlerts(error_messages: ReadSignal<ErrorMessages>) -> impl IntoView {
    view! {
        <div class="toast toast-top toast-end">
        {
            move || error_messages.get().as_ref().into_iter().map(|error_message| view! {
                <div class="alert alert-error alert-sm shadow-lg">
                    <div>
                        <svg xmlns="(link unavailable)" fill="none" viewBox="0 0 24 24" class="stroke-current inline-block w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                        </svg>
                        <span>{format!("{}", error_message)}</span>
                    </div>
                </div>
            }).collect_view()
        }
        </div>
    }
}
