use druid::{widget::Button, Data, FileDialogOptions, FileSpec, Lens, Widget};
#[derive(Clone, Data, Lens)]
struct MainMenuState {
    file_path: String,
}
fn build_menu() -> impl Widget<MainMenuState> {
    // druid::widget::Button
    let markdown_files = FileSpec::new("Markdown Dosyaları", &["md"]);
    let file_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![markdown_files])
        .title("Dosya Seç...");
    let open_file = Button::new("Dosya Aç").on_click(|ctx, data| {});
}
