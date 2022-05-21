use druid::{
    commands,
    text::{ArcStr, RichText, RichTextBuilder},
    widget::{Button, Flex},
    Command, Data, FileDialogOptions, FileSpec, Lens, Widget, WidgetExt,
};

pub fn build_menu() -> impl Widget<GeneralState> {
    // druid::widget::Button
    let markdown_files = FileSpec::new("Markdown Dosyaları", &["md"]);
    let text_files = FileSpec::new("Text Dosyaları", &["txt"]);
    let file_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![markdown_files, text_files])
        .title("Dosya Seç...");
    let open_file_button =
        Button::new("Dosya Aç").on_click(move |ctx, data: &mut GeneralState, _env| {
            ctx.submit_command(commands::SHOW_OPEN_PANEL.with(file_dialog_options.clone()));
        });
    let new_file_button = Button::new("Yeni Dosya");

    Flex::column()
        .with_child(open_file_button)
        .with_child(new_file_button)
}
pub fn open_file_menu_dialog() -> FileDialogOptions {
    let markdown_files = FileSpec::new("Markdown Dosyaları", &["md"]);
    let text_files = FileSpec::new("Text Dosyaları", &["txt"]);
    let file_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![markdown_files, text_files])
        .title("Dosya Seç...");
    return file_dialog_options;
}
/// General state to structure app data.
#[derive(Data, Lens, Clone)]
pub struct GeneralState {
    pub file_content_raw: String,
    pub file_name: String,
    pub is_on_menu: bool,
    pub file_path: String,
    pub raw: String,
    pub rendered: RichText,
}
