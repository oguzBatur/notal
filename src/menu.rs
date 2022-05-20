use std::fmt::Debug;

use druid::{
    commands,
    widget::{Button, Flex},
    Command, Data, FileDialogOptions, FileSpec, Lens, Widget,
};
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
    let open_file_button =
        Button::new("Dosya Aç").on_click(move |ctx, data: &mut MainMenuState, env| {
            ctx.submit_command(commands::SHOW_OPEN_PANEL.with(file_dialog_options.clone()));
        });
    let new_file_button = Button::new("Yeni Dosya");

    Flex::column()
        .with_child(open_file_button)
        .with_child(new_file_button)
}
