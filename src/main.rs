// Local crates.
mod inputs;
mod markdown;
mod ui;
use ui::ui_builder;
mod windows;
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};
fn main() {
    let main_window = WindowDesc::new(ui_builder());
    let data = 0_u32;
    AppLauncher::with_window(main_window).launch(data);
}
