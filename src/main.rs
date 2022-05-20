use std::path::PathBuf;

mod menu;
mod text_buffer;
use druid::{
    widget::{Button, Flex, Label},
    AppDelegate, AppLauncher, Data, Lens, LocalizedString, PlatformError, Widget, WidgetExt,
    WindowDesc,
};
fn main() {
    let window: WindowDesc<GeneralState> = WindowDesc::new(root_builder);
    let initial_state = GeneralState::new();
    AppLauncher::with_window(window)
        .launch(initial_state)
        .expect("Error occured.");
}

fn root_builder() -> impl Widget<GeneralState> {
    let text: LocalizedString<GeneralState> = LocalizedString::new("Hello World");
    let label: Label<GeneralState> = Label::new(text);
    return label;
}

#[derive(Data, Lens, Clone)]
struct GeneralState {
    file_path: Option<String>,
    file_content: Option<String>,
    file_name: Option<String>,
    is_on_menu: bool,
}
impl GeneralState {
    fn new() -> Self {
        Self {
            is_on_menu: true,
            file_content: None,
            file_name: None,
            file_path: None,
        }
    }
}

// Druid works like this,
// *event*: Event arrives from the operating system, such as a key press, a mouse movement, or a timer firing.
// *update*: After this call returns, the framework checks to see if the data was mutated. If so , it call the root widget's update method.
