use std::fs;
use std::time::Duration;
pub mod menu;
use log::error;
use menu::{build_menu, open_file_menu_dialog, GeneralState, NEW_FILE_SELECTOR};
mod text_buffer;
use crate::text_buffer::TextBufferData;
use druid::widget::{Axis, Container, Flex, Label, LensWrap, TabInfo, Tabs, TabsEdge, TabsPolicy, TabsTransition, ViewSwitcher};
use druid::{commands, im::Vector, LensExt, widget::{prelude::*, BackgroundBrush, Either, Split}, widget::{LineBreaking, RawLabel, Scroll, TextBox}, AppDelegate, AppLauncher, Color, Data, Env, Handled, Lens, LocalizedString, Menu, MenuItem, Selector, Widget, WidgetExt, WindowDesc, WindowId, lens };
use druid::text::RichText;
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
        tab_config: TabConfig {
            transition: TabsTransition::Slide(Duration::from_millis(250).as_nanos() as u64),
            axis: Axis::Horizontal,
            edge: TabsEdge::Leading,
        },
        raw: EMPTY_BUFFER_TEXT.to_owned(),
        rendered: rebuild_rendered_text(EMPTY_BUFFER_TEXT),
        advanced: DynamicTextBufferTab::new(1),
    };

    AppLauncher::with_window(window)
        .log_to_console()
        .delegate(Delegate)
        .launch(initial_state)
        .expect("Uygulamayı acma eylemi basarisiz");
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
            let file_contents = fs::read_to_string(data.file_path.clone()).unwrap();
            let file_path = file_info.path().to_str().unwrap().to_string();
            let file_name = file_info.path().file_name().unwrap().to_str().unwrap().to_owned();
            let raw = file_contents.clone();
            let rendered = rebuild_rendered_text(&raw.as_str());
            data.advanced.add_text_buffer_tab(file_path, file_name, raw, rendered );

            if data.is_on_menu {
                data.is_on_menu = false;
            }
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
    let menu = build_menu().background(BackgroundBrush::Color(Color::WHITE));
    let vs = ViewSwitcher::new(
        |app_s: &GeneralState, _| app_s.tab_config.clone(),
        |tc: &TabConfig, _,_| Box::new(build_tab_widget(tc)),
    );
    let either: Either<GeneralState> = Either::new(|data, _env| data.is_on_menu, menu, vs);
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
//TODO -

/// Dinamik Text Buffer sekmeleri oluşturup kontrol etmek için kullanılan struct.
#[derive(Clone, Data, Lens)]
pub struct DynamicTextBufferTab {
    hightest_tab: usize,
    removed_tabs: usize,
    text_buffers: Vector<TextBufferData>,
    tab_labels: Vector<usize>,
    current_buffer: TextBufferData,
}

impl DynamicTextBufferTab {
    /// Create a new DynamicTabData struct.
    fn new(hightest_tab: usize) -> Self {
        let empty_buffer = TextBufferData{
            rendered: rebuild_rendered_text(""),
            file_name: "".to_string(),
            file_path: "".to_string(),
            is_live_preview_open: true,
            raw: "".to_string(),
            key: 0

        };
        Self {
            hightest_tab,
            removed_tabs: 0,
            tab_labels: (1..=hightest_tab).collect(),
            text_buffers: Vector::from(vec![empty_buffer.clone()]),
           current_buffer: empty_buffer
        }
    }

    /// Yeni bir text buffer sekmesi yarat.
    fn add_text_buffer_tab(&mut self, file_path: String, file_name: String, raw: String, rendered: RichText ) {
        self.hightest_tab += 1;
        self.tab_labels.push_back(self.hightest_tab);
        self.text_buffers.push_back(TextBufferData {
            raw: raw.clone(),
            file_path,
            file_name,
            is_live_preview_open: true,
            rendered: rebuild_rendered_text(raw.as_str()),
            key: self.hightest_tab - 1
        });
    }

    /// Text Buffer Sekmesini Kapat.
    fn remove_text_buffer_tab(&mut self, idx: usize) {
        if idx >= self.tab_labels.len() {
            // ! Var olmayan sekme id'si ile işlem yapılmaya çalışılındı */
            error!("Var olmayan sekme kapatilmaya çalişilindi: {}", idx);
        } else {
            self.removed_tabs += 1;
            self.tab_labels.remove(idx);
            self.text_buffers.remove(idx);
        }
    }

    fn tabs_key(&self) -> (usize, usize) {
        return (self.hightest_tab, self.removed_tabs);
    }
}
#[derive(Data, Clone, Lens)]
pub struct TabConfig {
    pub axis: Axis,
    pub edge: TabsEdge,
    pub transition: TabsTransition,
}

/// Policy to control tabs dynamically.
#[derive(Clone, Data)]
struct TextBufferTabs;

impl TabsPolicy for TextBufferTabs {
    type Key = usize;
    type Build = ();
    type Input = DynamicTextBufferTab;
    type LabelWidget = Label<DynamicTextBufferTab>;
    type BodyWidget = Container<DynamicTextBufferTab>;

    fn tabs_changed(&self, old_data: &Self::Input, data: &Self::Input) -> bool {
        old_data.tabs_key() != data.tabs_key()
    }
    fn tabs(&self, data: &Self::Input) -> Vec<Self::Key> {
        data.tab_labels.iter().copied().collect()
    }
    fn tab_info(&self, key: Self::Key, _data: &Self::Input) -> TabInfo<Self::Input> {
        TabInfo::new(format!("{}", _data.text_buffers.get(key).unwrap().file_name), true)
    }
    fn close_tab(&self, key: Self::Key, data: &mut Self::Input) {
        if let Some(id) = data.tab_labels.index_of(&key) {
            data.remove_text_buffer_tab(id)
        }
    }
    fn tab_label(
        &self,
        _key: Self::Key,
        info: TabInfo<Self::Input>,
        _data: &Self::Input,
    ) -> Self::LabelWidget {
        Self::default_make_label(info)
    }
    fn tab_body(&self, key: Self::Key, _data: &Self::Input) -> Self::BodyWidget {
        let buffer_lens = lens!(DynamicTextBufferTab, text_buffers);
        buffer_lens.map(|txt| {
            txt.get_mut(key).unwrap()
        }, |txt, x|)

    }
}

fn build_tab_widget(tab_config: &TabConfig) -> impl Widget<GeneralState> {
    let dynamic_tabs = Tabs::for_policy(TextBufferTabs)
        .with_axis(tab_config.axis)
        .with_edge(tab_config.edge)
        .with_transition(tab_config.transition)
        .lens(GeneralState::advanced);
    Container::new(dynamic_tabs)
}

/// ### Builds the Text Buffer Widget that contains all the relevant components.
fn build_text_buffer_widget() -> impl Widget<TextBufferData> {
    let textbox = TextBox::multiline()
        .lens(TextBufferData::raw)
        .controller(RichTextRebuilder)
        .padding(5.0)
        .background(BackgroundBrush::Color(Color::WHITE));

    let textbox_standalone = TextBox::multiline()
        .lens(TextBufferData::raw)
        .controller(RichTextRebuilder)
        .padding(5.0)
        .background(BackgroundBrush::Color(Color::WHITE));

    let label = Scroll::new(
        RawLabel::new()
            .with_text_color(Color::BLACK)
            .with_line_break_mode(LineBreaking::WordWrap)
            .lens(TextBufferData::rendered)
            .padding(5.0),
    )
        .background(BackgroundBrush::Color(Color::SILVER));
    let either_text_buffer: Either<TextBufferData> = Either::new(
        |data, _env| data.is_live_preview_open,
        Split::columns(textbox, label)
            .draggable(true)
            .split_point(0.4),
        textbox_standalone,
    );

    return Container::new(either_text_buffer);
}