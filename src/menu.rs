use druid::{
    commands,
    text::RichText,
    widget::{Button, Flex},
    Data, FileDialogOptions, FileSpec, Lens, Selector, Widget,
};
pub const NEW_FILE_SELECTOR: Selector<EmptyFile> = Selector::new("notal.open.new_file");
type EmptyFile = bool;
pub fn build_menu() -> impl Widget<GeneralState> {
    // druid::widget::Button
    let markdown_files = FileSpec::new("Markdown Dosyaları", &["md"]);
    let text_files = FileSpec::new("Text Dosyaları", &["txt"]);
    let file_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![markdown_files, text_files])
        .title("Dosya Seç...");
    let open_file_button =
        Button::new("Dosya Aç").on_click(move |ctx, _data: &mut GeneralState, _env| {
            ctx.submit_command(
                commands::SHOW_OPEN_PANEL
                    .with(file_dialog_options.clone().name_label("Denem here.")),
            );
        });
    let new_file_button =
        Button::new("Yeni Dosya").on_click(move |ctx, _data: &mut GeneralState, _env| {
            ctx.submit_command(NEW_FILE_SELECTOR.with(true));
        });

    Flex::column()
        .with_child(open_file_button)
        .with_child(new_file_button)
}
pub fn open_file_menu_dialog() -> FileDialogOptions {
    let markdown_files = FileSpec::new("Markdown Dosyaları", &["md"]);
    let text_files = FileSpec::new("Text Dosyaları", &["txt"]);
    let file_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![markdown_files, text_files])
        .button_text("Aç")
        .title("Dosya Seç...");
    return file_dialog_options;
}

pub fn get_folder_dialog_options() -> FileDialogOptions {
    FileDialogOptions::new()
        .select_directories()
        .title("Dosya aç")
}
pub fn save_file_dialog() -> FileDialogOptions {
    let file_dialog_options = FileDialogOptions::new()
        .title("Dosya Kaydet...")
        .button_text("Kaydet");

    return file_dialog_options;
}
/// General state to structure app data.
#[derive(Data, Lens, Clone)]
pub struct GeneralState {
    pub file_content_raw: String,
    pub file_name: String,
    pub window_size: (f64, f64),
    pub is_on_menu: bool,
    pub file_path: String,
    pub raw: String,
    pub rendered: RichText,
    pub is_new_file: bool,
    pub is_live_preview_open: bool,
}
