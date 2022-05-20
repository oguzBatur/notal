use std::path::PathBuf;

mod menu;
mod text_buffer;
use druid::{
    widget::{Button, Flex, Label},
    AppDelegate, AppLauncher, Widget,LocalizedString, PlatformError WidgetExt, WindowDesc, Data, Lens,
};
fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(WindowDesc::new(root_builder())).launch(())?;
    Ok(())
}

fn root_builder() -> impl Widget<GeneralState> {
    let text:LocalizedString<_> = LocalizedString::new("Hello World");
    Label::new(text)
}

#[derive(Data, Lens, Clone)]
struct GeneralState{
    file_path: String,
    file_content: String,
    file_name: String
}

// Druid works like this,
// *event*: Event arrives from the operating system, such as a key press, a mouse movement, or a timer firing.
// *update*: After this call returns, the framework checks to see if the data was mutated. If so , it call the root widget's update method.
