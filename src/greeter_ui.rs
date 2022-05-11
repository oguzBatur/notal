use druid::widget::{
    prelude::*, Align, Button, Container, FillStrat, Flex, Image, Label, LabelText, Padding,
    Painter,
};
use druid::{
    theme, AppDelegate, Color, Data, Env, FileDialogOptions, FileInfo, FileSpec, ImageBuf, Insets,
    Lens, Selector, UnitPoint, Widget, WidgetExt, WindowHandle,
};

//* Event Handler */
pub fn notal_greeter_ui() -> impl Widget<String> {
    //* Dosya Seçme Eyleminde, desteklenen dosya tipleri için ürettiğimiz değerler. */
    let defaut_save_name = String::from("benim_dosyam");
    let text_file_spec = FileSpec::new("Text Dosyaları", &["txt"]);
    let html_file_spec = FileSpec::new("HTML", &["html"]);
    let markdown_file_spec = FileSpec::new("Markdown", &["md"]);
    let open_file_options = FileDialogOptions::new()
        .allowed_types(vec![text_file_spec, html_file_spec, markdown_file_spec])
        .button_text("Aç")
        .title(String::from("Dosya Aç"))
        .default_name(defaut_save_name)
        .default_type(markdown_file_spec);
    //* Implementasyon buraya */
    //? Programımızın girişinde logomuzu, dosya açıcımızı görmemiz tatlı olur.
    //TODO Label widgeti yarat.
    let greeter_label: Label<String> =
        Label::new(|_data: &String, _env: &Env| format!("Notal'a Hoşgeldin"));
    //TODO Image widgeti yarat.
    let new_file_button = create_default_notal_button("Yeni Dosya");
    let open_file_button = create_default_notal_button("Dosya Aç").on_click(move |ctx, _, _| {
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_file_options.clone()))
    });
    let container = Container::new(
        Flex::column()
            .with_child(Padding::new(Insets::new(5.0, 5.0, 5.0, 5.0), greeter_label))
            .with_child(Padding::new(
                Insets::new(5.0, 5.0, 5.0, 5.0),
                open_file_button,
            ))
            .with_child(Padding::new(
                Insets::new(5.0, 5.0, 5.0, 5.0),
                new_file_button,
            )),
    );
    //* Uygulamanın Arkaplan Rengi. */
    Align::new(UnitPoint::CENTER, container).background(Color::WHITE)
}
#[derive(Clone, Data, Lens)]
struct NotalButton {
    name: String,
}

//* Genel Notal tuşu üret */
fn create_default_notal_button(text: &str) -> Container<String> {
    let painter: Painter<String> = Painter::new(|ctx, _, env| {
        let bounds = ctx.size().to_rect();
        ctx.fill(bounds, &env.get(theme::PRIMARY_DARK));
        if ctx.is_hot() {
            ctx.stroke(bounds.inset(-0.5), &Color::BLUE, 1.0);
        }

        if ctx.is_active() {
            ctx.fill(bounds, &env.get(theme::PRIMARY_LIGHT))
        }
    });

    let container = Label::new(text)
        .with_text_color(Color::WHITE)
        .background(painter);
    container
}

#[derive(Clone, Data, Lens)]
pub struct GreeterState {
    pub name: String,
    pub open_file_name: String,
    pub new_file_name: String,
}

#[derive(Clone, Data, Lens)]
struct FileOpener {}

#[derive(Clone, Data, Lens)]
pub struct WindowMenu {}
