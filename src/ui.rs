use druid::widget::{Button, Flex, Label};
use druid::{LocalizedString, Widget, WidgetExt};

pub fn ui_builder() -> impl Widget<u32> {
    let text =
        LocalizedString::new("Hello-Counter").with_arg("count", |data: &u32, _env| (*data).into());
    let label = Label::new(text).padding(5.0).center();
    let button = Button::new("increment")
        .on_click(|_ctx, data, _env| *data += 1)
        .padding(5.0);
    Flex::column().with_child(label).with_child(button)
}
