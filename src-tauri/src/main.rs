#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod markdown;
use markdown::MarkdownBuilder;

#[tauri::command]
fn format_text(input:String) -> String {
    let mut builder = MarkdownBuilder::new(input);
    builder.format_to_html()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![format_text])
        .run(tauri::generate_context!())
        .expect("error while running notal");
}
