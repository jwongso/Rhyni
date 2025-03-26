use slint::SharedString;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

mod utils;
mod websocket_client;
use websocket_client::start_websocket_client;

slint::include_modules!();

fn main() {
    let app = App::new().unwrap();

    // Shared state for UI updates
    let transcription_input = Arc::new(Mutex::new(SharedString::default()));
    let selected_prompt = Arc::new(Mutex::new(SharedString::default()));
    let processed_output = Arc::new(Mutex::new(SharedString::default()));

    // Start WebSocket client in async runtime
    let rt = Runtime::new().unwrap();
    let ws_transcription_input = transcription_input.clone();
    let ws_processed_output = processed_output.clone();
    rt.spawn(async move {
        start_websocket_client().await;
    });

    // Set UI bindings
    app.global::<App>()
    .set_transcription_input(transcription_input.lock().unwrap().clone());
    app.global::<App>()
    .set_selected_prompt(selected_prompt.lock().unwrap().clone());
    app.global::<App>()
    .set_processed_output(processed_output.lock().unwrap().clone());

    // Handle button click (Send Selected Prompt)
    let ws_selected_prompt = selected_prompt.clone();
    let ws_processed_output = processed_output.clone();
    app.global::<App>().on_send_selection(move || {
        let selected_text = ws_selected_prompt.lock().unwrap().clone();
        *ws_processed_output.lock().unwrap() = selected_text.clone();
        println!("🚀 Sent to GPT: {}", selected_text);
    });

    app.run().unwrap();
}
