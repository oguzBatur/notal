use crate::{Color, DynamicTextBufferTab, TabConfig};
use druid::{Insets, LinearGradient, LocalizedString};
use druid::widget::{BackgroundBrush, FillStrat, Image, Label, SizedBox};
use druid::{
    commands, widget::Flex, Data, FileDialogOptions, FileSpec, FontDescriptor, FontWeight,
    ImageBuf, Lens, Selector, UnitPoint, Widget, WidgetExt,
};

pub const NEW_FILE_SELECTOR: Selector<EmptyFile> = Selector::new("notal.open.new_file");
type EmptyFile = bool;
/// ### Default Background Color of the menu.
const MENU_BUTTON_COLOR_1: Color = Color::rgb8(45,181,167);
const MENU_BUTTON_COLOR_2: Color = Color::rgb8(43,149,171);
const MENU_BACKGROUND_COLOR_1: Color = Color::rgb8(33,176,116);
const MENU_BACKGROUND_COLOR_2: Color = Color::rgb8(29,153,110);

/// ### Main Menu of the Notal App.
pub fn build_menu() -> impl Widget<GeneralState> {
    //? Get Notal Png as a byte array from root folder. Using expect to handle errors better.
    let localized_welcome:LocalizedString<GeneralState> = LocalizedString::new("label.welcome-message").with_placeholder("Welcome to Notal");
    let png_data = ImageBuf::from_data(include_bytes!("../notal_logo.png")).expect("Can't upload png logo");
    let img = Image::new(png_data.clone())
        .fill_mode(FillStrat::Fill)
        .padding(10.0);
    let sized: SizedBox<GeneralState> = SizedBox::new(img);

    let markdown_files = FileSpec::new("Markdown Dosyaları", &["md"]);
    let text_files = FileSpec::new("Text Dosyaları", &["txt"]);
    let file_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![markdown_files, text_files])
        .title("Dosya Seç...");

    let open_button = Label::new("Dosya Aç...")
        .with_font(FontDescriptor::with_weight(
            FontDescriptor::default(),
            FontWeight::BOLD,
        ))
        .with_text_size(15.0)
        .center()
        .background(MENU_BUTTON_COLOR_1)
        .rounded(4.0)
        .fix_size(200.0, 35.0)
        .on_click(move |ctx, _data: &mut GeneralState, _env| {
            ctx.submit_command(
                commands::SHOW_OPEN_PANEL.with(file_dialog_options.clone().name_label("Dosya aç")),
            );
        });

    let new_file_button = Label::new("Yeni Dosya")
        .with_font(FontDescriptor::with_weight(
            FontDescriptor::default(),
            FontWeight::BOLD,
        ))
        .with_text_size(15.0)
        .center()
        .background(MENU_BUTTON_COLOR_2)
        .rounded(4.0)
        .fix_size(200.0, 35.0)
        .on_click(move |ctx, _data: &mut GeneralState, _env| {
            ctx.submit_command(NEW_FILE_SELECTOR.with(true));
        })
        .padding(10.0);

    let landing_text = Label::new(localized_welcome)
        .with_font(FontDescriptor::with_weight(FontDescriptor::default(), FontWeight::EXTRA_BOLD))
        .with_text_size(22.0)
        .padding(Insets::new(0.0, 0.0, 0.0, 20.0));

    let button_flex_column = Flex::column()
        .with_child(landing_text)
        .with_child(open_button)
        .with_child(new_file_button)
        .padding(Insets::new(100.0, 10.0, 10.0, 10.0))
        .align_vertical(UnitPoint::CENTER)
        .align_horizontal(UnitPoint::CENTER);

    // Get 5 percent of the original width and height of the logo.
    let png_width = png_data.width() as f64 * 0.06;
    let png_height = png_data.height() as f64 * 0.06;

    Flex::row()
        .with_child(sized.width(png_width).height(png_height))
        .with_child(button_flex_column)
        .align_horizontal(UnitPoint::CENTER)
        .align_vertical(UnitPoint::CENTER)
        .background(BackgroundBrush::Linear(LinearGradient::new(UnitPoint::CENTER, UnitPoint::RIGHT, (MENU_BACKGROUND_COLOR_1, MENU_BACKGROUND_COLOR_2))))
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
}
