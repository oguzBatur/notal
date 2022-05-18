use iced::{
    button, window, Application, Button, Column, Container, Element, Font, Padding, Row, Settings,
    Svg, Text, TextInput,
};
use ropey;

use native_dialog::FileDialog;
use std::{fs, path::PathBuf};
pub fn main() -> iced::Result {
    let external_font = include_bytes!("../fonts/static/Inter-Medium.ttf");
    NotalApp::run(Settings {
        id: Some(String::from("notal_app")),
        window: iced::window::Settings {
            size: (800, 600),
            always_on_top: false,
            decorations: true,
            resizable: true,
            transparent: false,
            position: window::Position::Centered,
            max_size: Some((1920, 1080)),
            min_size: Some((800, 600)),
            icon: None,
            ..Default::default()
        },
        flags: (),
        default_font: Some(external_font),
        default_text_size: 20,
        text_multithreading: false,
        antialiasing: true,
        exit_on_close_request: true,
        try_opengles_first: false,
    })
}

//* Create an image . */
//* Root of our project. implement application in here. */
/// The root of the NotalApp
pub enum NotalApp {
    Loading(MainMenu),
    Loaded(Panes), //* Bu noktada, input bufferımız açılmış olmalı. */
}

/// Mesajlar, update fonksiyonlarında kontrol edilir ve buna göre işlem yapılır
/// Mesajlar, ui elementlerinin içerisinde kullanılabilir. butona basıldığında şu mesajı yolla gibi.
/// Dene Yapmak iyidir bilirsin
#[derive(Debug, Clone)]
pub enum Message {
    OpenBuffer, //* Burada, uygulamada, yüklü bir dosya varmı yokmu o belli olacak., Loaderror, bize FileDialogState'i vermeli.
    CloseBuffer,
    OpenMenu,
    MainMenuMessage(MainMenuMessage),
    PaneMessage(PaneState),
}
/// Genel mimari;
/// eğer uygulamada önceden bir dosya açılmışsa, o zaman dosya aç kısmını geç.
/// Eğer açılmamışsa, ana menüyü göster. Ana menüde ise, dosya açma tuşu, ve yeni dosya açma tuşu ile beraber bir logo bulunmalı.
/// Menü için yapılması gerekenler. Root'un Update kısmında, Eğer applikasyon loading'te ise, Loaded yap. bunu yaparken iki seçenek var. ya uygulama yüklenir yada yüklenmez.
/// Yüklenmez ise, hata verir ve menüyü açma güncellemesi yapar.
/// Root'un view metodunda, gelen mesaj, Error içeriyorsa, bizim FileDialogState'in implementasyonunu yükle.
/// Update Kısmı, Alıcı Kısım
/// View Kısmındaki UI elementleri, Vericiler.
/// Update, state'i değiştirmekten sorumlumu ? evet. State değilde command yollasa o nereeye gider ?
///
impl Application for NotalApp {
    type Flags = ();
    type Message = Message;
    type Executor = iced::executor::Default;
    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (NotalApp::Loading(MainMenu::new()), iced::Command::none())
    }
    fn title(&self) -> String {
        String::from("Notal")
    }
    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match self {
            NotalApp::Loading(menu) => match message {
                Message::MainMenuMessage(main_msg) => {
                    //* Mainmenu'ye gelen mesajları mainmenu update metodune parenttan yolladık. */
                    menu.update(main_msg.clone());
                    //* Eğer mesaj, OpenFilePressed ise, o zaman önce girilen path doğrumu ona bak, doğru ise, TextBufferı bize ver. */
                    if let MainMenuMessage::OpenFilePressed = &main_msg {
                        if menu.file_path.exists() {
                            println!("{:?}", &menu.file_path);
                            *self = NotalApp::Loaded(Panes::new(menu.file_path.clone(), None));
                        }
                    }
                }
                _ => (),
            },
            NotalApp::Loaded(pane) => match message {
                Message::PaneMessage(pane_msg) => {
                    pane.update(pane_msg);
                }
                _ => (),
            },
        }
        iced::Command::none()
    }
    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        match self {
            //* Bir problem çözüldü. Anlaşılan, Mesajlari birbirine chainlememiz gerekiyor, ve view içerisinde kullanılacak custom widget'in view komutunun kullanılması gerekiyor  */
            NotalApp::Loading(menu) => Column::new()
                .push(menu.view().map(Message::MainMenuMessage))
                .align_items(iced::Alignment::Center)
                .height(iced::Length::Fill)
                .width(iced::Length::Fill)
                .into(),
            NotalApp::Loaded(panes) => Container::new(panes.view().map(Message::PaneMessage))
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .into(),
        }
    }
}

///MainMenu for **Elm - *State* architecture**
#[derive(Debug, Clone)]
pub struct MainMenu {
    file_dialog_opened: bool,
    open_file_button: button::State,
    new_file_button: button::State,
    file_path: PathBuf,
}

///MainMenuMessage for **Elm - *Message* architecture**
#[derive(Clone, Debug)]
pub enum MainMenuMessage {
    OpenFilePressed,
    NewFilePressed,
}
impl<'a> MainMenu {
    fn new() -> Self {
        MainMenu {
            file_dialog_opened: false,
            open_file_button: iced::button::State::new(),
            new_file_button: iced::button::State::new(),
            file_path: PathBuf::new(),
        }
    }
    fn update(&mut self, msg: MainMenuMessage) {
        match msg {
            MainMenuMessage::NewFilePressed => {
                println!("New File Pressed");
            }
            MainMenuMessage::OpenFilePressed => {
                self.file_dialog_opened = true;
                match file_dialog_opener(FileDialogCommands::SHOW_OPEN_FILE_DIALOG) {
                    Ok(path) => {
                        self.file_path = path.clone();
                    }
                    Err(err) => println!("There is an error: {:?}", err),
                }
                self.file_dialog_opened = false;
                println!("{}", self.file_dialog_opened);
            }
        }
    }
    fn view(&mut self) -> Element<MainMenuMessage> {
        let bold_font = include_bytes!("../fonts/static/Inter-Bold.ttf");
        let svg = Svg::from_path(PathBuf::from("src/NotalLogo.svg"))
            .width(iced::Length::Units(150))
            .height(iced::Length::Units(150));
        let text = Text::new("Notal")
            .size(30)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .vertical_alignment(iced::alignment::Vertical::Center)
            .font(Font::External {
                name: "Inter-Bold",
                bytes: bold_font,
            });

        let column: Element<MainMenuMessage> = Column::new()
            .push(svg)
            .push(text)
            .align_items(iced::Alignment::Center)
            .into();
        Column::new()
            .push(column)
            .padding(Padding::from(20))
            .spacing(10)
            .push(
                Button::new(
                    &mut self.open_file_button,
                    Text::new("Dosya Aç")
                        .horizontal_alignment(iced::alignment::Horizontal::Center)
                        .vertical_alignment(iced::alignment::Vertical::Center),
                )
                .on_press(MainMenuMessage::OpenFilePressed)
                .width(iced::Length::Units(100))
                .height(iced::Length::Units(30))
                .style(style::Button::Primary),
            )
            .push(
                Button::new(
                    &mut self.new_file_button,
                    Text::new("Yeni Dosya")
                        .horizontal_alignment(iced::alignment::Horizontal::Center)
                        .vertical_alignment(iced::alignment::Vertical::Center),
                )
                .on_press(MainMenuMessage::NewFilePressed)
                .width(iced::Length::Units(100))
                .height(iced::Length::Units(30))
                .style(style::Button::Secondary),
            )
            .align_items(iced::Alignment::Center)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

/// Checks if the file given is in the true format.
#[derive(Debug, Clone)]
enum LoadError {
    FileError,
    FormatError,
}

/// [Panes] - *Message*
#[derive(Debug, Clone)]
pub enum PaneState {
    TextBufferMessage(TextBufferMessage),
    FileTreeMessage(FileTreeMessage),
}
/// Initilize panes when mainmenu is closed.
pub struct Panes {
    file_tree: FileTree,
    text_buffer: TextBuffer,
}

#[derive(Debug, Clone)]
pub struct FileTree {
    folder_path: PathBuf,
    dictionary_state: button::State,
    dictionary_terms: Option<Vec<DictionaryItem>>,
}

#[derive(Debug, Clone)]
pub enum FileTreeMessage {
    OpenFile(PathBuf),
    OpenDictionary(PathBuf),
    DictionaryTermSelected(DictionaryItem),
    TextBufferMessage(TextBufferMessage),
}
///  A str type used to define terms <br> Terimleri  anlatmak için kullandığımız bir str türü.
type Term = String;
///  A str type used to define term descriptions <br> Terim açıklamalarını anlatmak için kullandığımız bir str türü.
type Description = String;
///  A tuple type used to store terms and their descriptions. <br> Kütüphanemizdeki terimleri ve açıklamalarını sakladığımız bir tuple.
type DictionaryItem = (Term, Description);

impl<'a> FileTree {
    fn new(path: PathBuf, dictionary_terms: Option<Vec<DictionaryItem>>) -> Self {
        FileTree {
            folder_path: path,
            dictionary_state: button::State::new(),
            dictionary_terms: match dictionary_terms {
                Some(terms) => Some(terms),
                None => None,
            },
        }
    }
    fn update(&mut self, msg: FileTreeMessage) {
        todo!();
    }

    fn view(&mut self) -> Element<FileTreeMessage> {
        //* The Dictionary Button. - Kütüphane Butonu. */
        let dictionary_button: Button<FileTreeMessage> = Button::new(
            &mut self.dictionary_state,
            Text::new("Kütüphanem")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center),
        )
        .style(style::Button::Primary)
        .on_press(FileTreeMessage::OpenDictionary(self.folder_path.clone()));
        let file_label = Text::new("file name goes here.");
        let column = Column::new()
            .push(dictionary_button)
            .push(file_label)
            .padding(20)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .spacing(10)
            .align_items(iced::Alignment::Center);

        Container::new(column)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .style(style::Pane::FileTreePane(style::PaneState::Active))
            .into()
    }
}
/// Commands for the FileDialog Opener.
#[allow(non_camel_case_types)]
enum FileDialogCommands {
    SHOW_SAVE_FILE_DIALOG(SaveFile),
    SHOW_OPEN_FILE_DIALOG,
}
struct SaveFile {
    dir: PathBuf,
}
/// Error Options for @MainMenu the dialog opener errors.
#[derive(Debug)]
enum FileDialogError {
    FileNotFoundError,
    SaveLocationNotFoundError,
}

fn file_dialog_opener(command: FileDialogCommands) -> Result<PathBuf, FileDialogError> {
    match command {
        FileDialogCommands::SHOW_OPEN_FILE_DIALOG => {
            let path = FileDialog::new()
                .set_location("~/Desktop")
                .add_filter("Markdown Dosyaları", &["md"])
                .add_filter("Text Dosyası", &["txt"])
                .show_open_single_file()
                .unwrap();
            match path {
                Some(path) => Ok(path),
                None => Err(FileDialogError::FileNotFoundError),
            }
        }
        FileDialogCommands::SHOW_SAVE_FILE_DIALOG(save_file) => {
            let path = FileDialog::new()
                .set_location("~/Desktop")
                .add_filter("Text Dosyası", &["txt"])
                .add_filter("Markdown Dosyaları", &["md"])
                .show_save_single_file()
                .unwrap();
            match path {
                Some(path) => {
                    println!("{:?}", path.to_str().unwrap());
                    Ok(path)
                }
                None => Err(FileDialogError::SaveLocationNotFoundError),
            }
        }
    }
}
/// General UI terminology
/// Dialog => A pop up window that appears after an action.
/// Field => An area in the WUI or GUI where you need to enter information.
/// Pane => An independent arera in the WUI or GUI that you can scroll and resize.
/// Wizard => A dialog that walks a user through the sequence of steps to perform a particular task.

#[derive(Debug, Clone)]
pub enum TextBufferMessage {
    NewInputEntry(String),
    ChangeFileRequested(PathBuf),
}
#[derive(Debug, Clone)]
pub struct TextBuffer {
    input_state: iced::text_input::State,
    input_value: String,
    file_contents: String,
    file_path: PathBuf,
}

impl<'a> TextBuffer {
    fn new(path: PathBuf) -> Self {
        let file = fs::read_to_string(path.clone()).unwrap();
        Self {
            input_state: iced::text_input::State::new(),
            file_contents: file,
            file_path: path,
            input_value: String::from(""),
        }
    }

    fn update(&mut self, msg: TextBufferMessage) {
        match msg {
            TextBufferMessage::ChangeFileRequested(path) => {
                self.file_path = path;
            }
            _ => (),
        }
    }
    fn view(&mut self) -> Element<TextBufferMessage> {
        Container::new(TextInput::new(
            &mut self.input_state,
            self.file_contents.as_str(),
            self.input_value.as_str(),
            TextBufferMessage::NewInputEntry,
        ))
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .style(style::Pane::TextBufferPane(style::PaneState::Active))
        .into()
    }
}

impl<'a> Panes {
    fn new(path: PathBuf, dict: Option<Vec<DictionaryItem>>) -> Self {
        Self {
            file_tree: FileTree::new(path.clone(), dict),
            text_buffer: TextBuffer::new(path),
        }
    }
    fn update(&mut self, msg: PaneState) {
        match msg {
            PaneState::FileTreeMessage(file_msg) => {
                self.file_tree.update(file_msg);
            }
            PaneState::TextBufferMessage(text_msg) => {
                self.text_buffer.update(text_msg);
            }
        }
    }
    fn view(&mut self) -> Element<PaneState> {
        let file_tree_container =
            Container::new(self.file_tree.view().map(PaneState::FileTreeMessage))
                .width(iced::Length::FillPortion(1))
                .height(iced::Length::Fill);
        let text_buffer_container =
            Container::new(self.text_buffer.view().map(PaneState::TextBufferMessage))
                .width(iced::Length::FillPortion(3))
                .height(iced::Length::Fill);
        Row::new()
            .push(file_tree_container)
            .push(text_buffer_container)
            .into()
    }
}

/// A simple styling library taken from the examples. <br>
///  Provides styling for **Button, Pane**.
mod style {
    use iced::button;
    use iced::{container, Background, Color, Vector};

    /// Generic style of a Notal button.
    pub enum Button {
        Primary,
        Secondary,
    }

    pub enum Pane {
        TextBufferPane(PaneState),
        FileTreePane(PaneState),
    }
    pub enum PaneState {
        Active,
        Focused,
    }
    //*  Default file tree pane colors. */
    const FILE_TREE_PANE_ACTIVE_COLOR: Color =
        Color::from_rgb(163.0 / 255.0, 213.0 / 255.0, 172.0 / 255.0);
    const FILE_TREE_PANE_FOCUSED_COLOR: Color =
        Color::from_rgb(255.0 / 28.0, 255.0 / 151.0, 255.0 / 109.0);

    pub enum TitleBar {
        Active,
        Focused,
    }
    impl container::StyleSheet for TitleBar {
        fn style(&self) -> container::Style {
            match self {
                TitleBar::Active => container::Style {
                    text_color: Some(Color::WHITE),
                    background: Some(Background::Color(FILE_TREE_PANE_ACTIVE_COLOR)),
                    ..Default::default()
                },
                TitleBar::Focused => container::Style {
                    text_color: Some(Color::WHITE),
                    background: Some(Background::Color(FILE_TREE_PANE_FOCUSED_COLOR)),
                    ..Default::default()
                },
            }
        }
    }
    //* Style sheet for panes. */
    impl container::StyleSheet for Pane {
        fn style(&self) -> container::Style {
            match self {
                Pane::FileTreePane(pane_state) => match pane_state {
                    PaneState::Active => container::Style {
                        background: Some(Background::Color(FILE_TREE_PANE_ACTIVE_COLOR)),
                        border_color: Color::BLACK,
                        border_radius: 0.0,
                        border_width: 1.0,
                        text_color: Some(DEFAULT_BLUE_COLOR),
                    },
                    PaneState::Focused => container::Style {
                        background: Some(Background::Color(FILE_TREE_PANE_FOCUSED_COLOR)),
                        border_color: Color::BLACK,
                        border_radius: 0.0,
                        border_width: 1.0,
                        text_color: Some(DEFAULT_BLUE_COLOR),
                    },
                },
                Pane::TextBufferPane(pane_state) => match pane_state {
                    PaneState::Active => container::Style {
                        background: Some(Background::Color(Color::WHITE)),
                        border_color: DEFAULT_GREEN_COLOR,
                        border_radius: 0.0,
                        border_width: 2.0,
                        text_color: Some(Color::from_rgb8(10, 10, 10)),
                    },
                    PaneState::Focused => container::Style {
                        background: Some(Background::Color(Color::from_rgb8(250, 250, 250))),
                        border_color: DEFAULT_BLUE_COLOR,
                        border_radius: 0.0,
                        border_width: 2.0,
                        text_color: Some(Color::BLACK),
                    },
                },
            }
        }
    }

    //* Default Colors for Notal. */
    const DEFAULT_BLUE_COLOR: Color = Color::from_rgb(0.11, 0.42, 0.87);
    const DEFAULT_GREEN_COLOR: Color = Color::from_rgb(0.5, 0.5, 0.5);

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => DEFAULT_BLUE_COLOR,
                    Button::Secondary => DEFAULT_GREEN_COLOR,
                })),
                border_radius: 2.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),

                ..button::Style::default()
            }
        }
        fn hovered(&self) -> button::Style {
            button::Style {
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 2.0),
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.92),
                    Button::Secondary => Color::from_rgb(0.5, 0.3, 0.5),
                })),
                ..self.active()
            }
        }
    }
}
