use std::fs;
pub mod menu;
use menu::{build_menu, open_file_menu_dialog, GeneralState, NEW_FILE_SELECTOR};
mod text_buffer;
use druid::{
    commands,
    widget::{prelude::*, BackgroundBrush, Either, Split},
    widget::{LineBreaking, RawLabel, Scroll, TextBox},
    AppDelegate, AppLauncher, Color, Data, Env, Handled, Lens, LocalizedString, Menu, MenuItem,
    Selector, Widget, WidgetExt, WindowDesc, WindowId,
};
use text_buffer::{rebuild_rendered_text, RichTextRebuilder};
/// Empy Buffer Text.
const EMPTY_BUFFER_TEXT: &str = "";
const WINDOW_TITLE: LocalizedString<GeneralState> = LocalizedString::new("Notal");
const CLOSE_BUFFER: Selector<()> = Selector::new("notal-close-buffer");
const OPEN_PREVIEW: Selector<()> = Selector::new("notal-close-preview");
const CLOSE_PREVIEW: Selector<()> = Selector::new("notal-open-preview");
///
const DEFAULT_WINDOW_SIZE: (f64, f64) = (800.0, 600.0);
fn main() {
    let window: WindowDesc<GeneralState> = WindowDesc::new(root_builder())
        .title(WINDOW_TITLE)
        .menu(make_menu)
        .window_size(Size::new(DEFAULT_WINDOW_SIZE.0, DEFAULT_WINDOW_SIZE.1));

    let initial_state = GeneralState {
        file_content_raw: "".to_string(),
        window_size: DEFAULT_WINDOW_SIZE,
        is_live_preview_open: true,
        is_new_file: false,
        file_name: "".to_string(),
        file_path: "".to_string(),
        is_on_menu: true,
        raw: EMPTY_BUFFER_TEXT.to_owned(),
        rendered: rebuild_rendered_text(EMPTY_BUFFER_TEXT),
    };

    AppLauncher::with_window(window)
        .log_to_console()
        .delegate(Delegate)
        .launch(initial_state)
        .expect("Error occured.");
}
struct Delegate;

impl AppDelegate<GeneralState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &druid::Command,
        data: &mut GeneralState,
        _env: &Env,
    ) -> druid::Handled {
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            // write the file path to state.
            data.file_path = file_info.path().to_str().unwrap().to_string();
            // Write the file name to state.
            data.file_name = file_info
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
            let file_contents = fs::read_to_string(data.file_path.clone()).unwrap();
            data.raw = file_contents.clone();
            data.rendered = rebuild_rendered_text(&file_contents.as_str());
            data.is_on_menu = false;

            Handled::Yes
        } else if let Some(state) = cmd.get(NEW_FILE_SELECTOR) {
            if state.to_owned() {
                data.is_on_menu = false;
                data.raw = "".to_string();
                data.rendered = rebuild_rendered_text("");
                Handled::Yes
            } else {
                Handled::No
            }
        } else if let Some(_) = cmd.get(CLOSE_BUFFER) {
            data.is_on_menu = true;
            data.raw = "".to_string();
            data.rendered = rebuild_rendered_text("");
            Handled::Yes
        } else if let Some(_) = cmd.get(CLOSE_PREVIEW) {
            data.is_live_preview_open = false;
            Handled::Yes
        } else if let Some(_) = cmd.get(OPEN_PREVIEW) {
            data.is_live_preview_open = true;
            Handled::Yes
        } else {
            Handled::No
        }
    }
}

/// Ui function that implements the root widget.
fn root_builder() -> impl Widget<GeneralState> {
    let textbox = TextBox::multiline()
        .lens(GeneralState::raw)
        .controller(RichTextRebuilder)
        .padding(5.0)
        .background(BackgroundBrush::Color(Color::WHITE));

    let textbox_standalone = TextBox::multiline()
        .lens(GeneralState::raw)
        .controller(RichTextRebuilder)
        .padding(5.0)
        .background(BackgroundBrush::Color(Color::WHITE));

    let label = Scroll::new(
        RawLabel::new()
            .with_text_color(Color::BLACK)
            .with_line_break_mode(LineBreaking::WordWrap)
            .lens(GeneralState::rendered)
            .padding(5.0),
    )
    .background(BackgroundBrush::Color(Color::SILVER));
    let menu = build_menu().background(BackgroundBrush::Color(Color::WHITE));
    let either_text_buffer: Either<GeneralState> = Either::new(
        |data, _env| data.is_live_preview_open,
        Split::columns(textbox, label)
            .draggable(true)
            .split_point(0.4),
        textbox_standalone,
    );

    let either: Either<GeneralState> =
        Either::new(|data, _env| data.is_on_menu, menu, either_text_buffer);
    return either;
}

/// Uygulama için düzenleme menüsü.
fn make_menu<T: Data>(
    _window_id: Option<WindowId>,
    _app_state: &GeneralState,
    _env: &Env,
) -> Menu<T> {
    let mut base = Menu::empty();
    #[cfg(target_os = "macos")]
    {
        base = base.entry(druid::platform_menus::mac::application::default())
    }
    #[cfg(any(target_os = "windows", target_os = "linux", target_os = "openbsd"))]
    {
        // a custom menu for notal.
        base = base.entry(
            Menu::new(LocalizedString::new("Dosya"))
                .entry(
                    MenuItem::new("Dosya Aç...")
                        .command(commands::SHOW_OPEN_PANEL.with(open_file_menu_dialog())),
                )
                .entry(MenuItem::new("Kapat").command(CLOSE_BUFFER.with(())))
                .entry(MenuItem::new("Canlı Önizlemeyi Aç").command(OPEN_PREVIEW.with(())))
                .entry(MenuItem::new("Canlı Önizlemeyi Kapat").command(CLOSE_PREVIEW.with(()))),
        );
    }
    base.entry(
        Menu::new(LocalizedString::new("Düzenle"))
            .entry(druid::platform_menus::common::undo())
            .entry(druid::platform_menus::common::redo())
            .separator()
            .entry(druid::platform_menus::common::cut())
            .entry(druid::platform_menus::common::copy())
            .entry(druid::platform_menus::common::paste()),
    )
}

// Druid works like this,
// *event*: Event arrives from the operating system, such as a key press, a mouse movement, or a timer firing.
// *update*: After this call returns, the framework checks to see if the data was mutated. If so , it call the root widget's update method.

//TODO - Add text buffer vectors to make it possible to create more than one text buffer.

#[derive(Clone, Data, Lens)]
struct DynamicTabData {}
