use std::path::PathBuf;
use druid::{commands, text::RichText, widget::{Button, Flex}, Data, FileDialogOptions, FileSpec, Lens, Selector, Widget, ImageBuf, WidgetExt};
use druid::widget::{BackgroundBrush, CrossAxisAlignment, FillStrat, FlexParams, Image, SizedBox, TabsEdge};
use crate::{Color, DynamicTextBufferTab, FileInfo, TabConfig, Vector};

pub const NEW_FILE_SELECTOR: Selector<EmptyFile> = Selector::new("notal.open.new_file");
type EmptyFile = bool;
/// ### Default Background Color of the menu.
const MENU_BACKGROUND_COLOR: Color = Color::rgb8(173, 216, 230);

/// ### Main Menu of the Notal App.
pub fn build_menu() -> impl Widget<GeneralState> {
    let png_data = ImageBuf::from_data(include_bytes!("../notal_logo.png")).unwrap();
    let img = Image::new(png_data).fill_mode(FillStrat::Fill);
    let sized:SizedBox<GeneralState> = SizedBox::new(img);
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
        .with_child(sized.width(200.0))
        .with_child(open_file_button)
        .with_child(new_file_button).background(BackgroundBrush::Color(MENU_BACKGROUND_COLOR))
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
    pub window_size: (f64, f64),
    pub is_on_menu: bool,
    pub is_new_file: bool,
    pub advanced: DynamicTextBufferTab,
    pub tab_config: TabConfig,
    pub files: Vector<PathBuf>

}
