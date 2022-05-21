use std::{fs, path::PathBuf};

pub mod menu;
use menu::{build_menu, open_file_menu_dialog, GeneralState};
mod text_buffer;
use druid::{
    commands,
    widget::{prelude::*, BackgroundBrush, Container, Either, Split},
    widget::{Label, LineBreaking, RawLabel, Scroll, TextBox},
    AppDelegate, AppLauncher, Color, Command, Data, Env, Handled, Lens, LocalizedString, Menu,
    PlatformError, Selector, Widget, WidgetExt, WindowDesc, WindowId,
};
use text_buffer::{rebuild_rendered_text, RichTextRebuilder};
/// Empy Buffer Text.
const EMPTY_BUFFER_TEXT: &str = "";
const WINDOW_TITLE: LocalizedString<GeneralState> = LocalizedString::new("Notal");
/// Command for opening links in markdown.
const OPEN_LINK: Selector<String> = Selector::new("druid-example.open-link");
fn main() {
    let window: WindowDesc<GeneralState> = WindowDesc::new(root_builder())
        .title(WINDOW_TITLE)
        .menu(make_menu)
        .window_size(Size::new(800.0, 600.0));

    let initial_state = GeneralState {
        file_content_raw: "".to_string(),
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
        ctx: &mut druid::DelegateCtx,
        target: druid::Target,
        cmd: &druid::Command,
        data: &mut GeneralState,
        env: &Env,
    ) -> druid::Handled {
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            data.file_path = file_info.path().to_str().unwrap().to_string();
            let file_contents = fs::read_to_string(data.file_path.clone()).unwrap();
            data.raw = file_contents.clone();
            data.rendered = rebuild_rendered_text(&file_contents.as_str());
            data.is_on_menu = false;

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
        .expand()
        .padding(5.0);
    let label = Scroll::new(
        RawLabel::new()
            .with_text_color(Color::BLACK)
            .with_line_break_mode(LineBreaking::WordWrap)
            .lens(GeneralState::rendered)
            .padding(5.0),
    );
    let menu = build_menu();
    let either: Container<GeneralState> = Either::new(
        |data, _env| data.is_on_menu,
        menu,
        Split::columns(textbox, label),
    )
    .background(BackgroundBrush::Color(Color::WHITE));
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
            druid::platform_menus::win::file::open()
                .command(commands::SHOW_OPEN_PANEL.with(open_file_menu_dialog())),
        );
    }
    base.entry(
        Menu::new(LocalizedString::new("common-menu-edit-menu"))
            .entry(druid::platform_menus::common::undo())
            .entry(druid::platform_menus::common::redo())
            .separator()
            .entry(druid::platform_menus::common::cut().enabled(false))
            .entry(druid::platform_menus::common::copy())
            .entry(druid::platform_menus::common::paste()),
    )
}
// Druid works like this,
// *event*: Event arrives from the operating system, such as a key press, a mouse movement, or a timer firing.
// *update*: After this call returns, the framework checks to see if the data was mutated. If so , it call the root widget's update method.
