use iced::{
    button,
    pane_grid::{self},
    window::{self},
    Application, Button, Column, Container, Element, Font, Padding, PaneGrid, Row, Settings, Svg,
    Text,
};
use native_dialog::FileDialog;
use std::path::PathBuf;
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
                            *self = NotalApp::Loaded(Panes::new(menu.file_path.clone()));
                        }
                    }
                }
                _ => (),
            },
            NotalApp::Loaded(panes) => match message {
                Message::PaneMessage(pane_msg) => {
                    panes.update(pane_msg);
                }
                _ => (),
            },
            _ => (),
        }
        iced::Command::none()
    }
    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        println!("Opened the main menu.");
        match self {
            //* Bir problem çözüldü. Anlaşılan, Mesajlari birbirine chainlememiz gerekiyor, ve view içerisinde kullanılacak custom widget'in view komutunun kullanılması gerekiyor  */
            NotalApp::Loading(menu) => Column::new()
                .push(menu.view().map(Message::MainMenuMessage))
                .align_items(iced::Alignment::Center)
                .height(iced::Length::Fill)
                .width(iced::Length::Fill)
                .into(),
            NotalApp::Loaded(panes) => {
                panes.view()
                todo!();
            }
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
    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    Clicked(pane_grid::Pane),
    Resized(pane_grid::ResizeEvent),
    Close(pane_grid::Pane),
}
/// Initilize panes when mainmenu is closed.
pub struct Panes {
    filetree_pane_state: pane_grid::State<FileTree>,
    textbuffer_pane_state: pane_grid::State<TextBuffer>,
    filetree_pane: pane_grid::Pane,
    textbuffer_pane: pane_grid::Pane,
}
struct FileTree {
    folder_path: PathBuf,
    dictionary_state: button::State,
    dictionary_terms: Option<Vec<DictionaryItem>>,
}
#[derive(Debug, Clone)]
pub enum FileTreeMessage {
    OpenFile(PathBuf),
    OpenDictionary(PathBuf),
    DictionaryTermSelected(DictionaryItem),
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

    fn view(&mut self, pane: iced::pane_grid::Pane) -> Element<FileTreeMessage> {
        //* The Dictionary Button. - Kütüphane Butonu. */
        let dictionary_button: Button<FileTreeMessage> = Button::new(
            &mut self.dictionary_state,
            Text::new("Kütüphanem")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center),
        )
        .on_press(FileTreeMessage::OpenDictionary(self.folder_path.clone()));
        let file_label = Text::new("file name goes here.");

        todo!()
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
pub struct TextBuffer {
    input_line: String,
    file_contents: String,
    file_path: PathBuf,
}
impl<'a> TextBuffer {
    fn new(path: PathBuf) -> Self {
        Self {
            input_line: "".to_string(),
            file_contents: "".to_string(),
            file_path: path,
        }
    }

    fn update(&mut self, msg: TextBufferMessage) {
        match msg {
            TextBufferMessage::NewInputEntry(new_input) => {
                self.input_line = new_input;
            }
            TextBufferMessage::ChangeFileRequested(path) => {
                self.file_path = path;
            }
        }
    }
    fn view(&mut self) -> Element<TextBufferMessage> {
        Column::new().push(Text::new("The Text BUffer")).into()
    }
}

impl<'a> Panes {
    fn new(path: PathBuf) -> Self {
        let filetree = FileTree::new(path.clone(), None);
        let textbuffer = TextBuffer::new(path);
        let filetree = pane_grid::State::new(filetree);
        let textbuffer = pane_grid::State::new(textbuffer);
        Self {
            filetree_pane_state: filetree.0,
            textbuffer_pane_state: textbuffer.0,
            filetree_pane: filetree.1,
            textbuffer_pane: textbuffer.1,
        }
    }
    fn update(&mut self, msg: PaneState) {
        match msg {
            PaneState::TextBufferMessage(txt_msg) => {
                self.textbuffer_pane_state
                    .get_mut(&self.textbuffer_pane)
                    .unwrap()
                    .update(txt_msg);
            }
            PaneState::FileTreeMessage(file_msg) => self
                .filetree_pane_state
                .get_mut(&self.filetree_pane)
                .unwrap()
                .update(file_msg),

            _ => (),
        }
    }
    fn view(&mut self, file_tree_pane: iced::pane_grid::Pane) -> Element<PaneState> {
        // Lets create a filetree pane first.
        let file_tree_pane_grid =
            PaneGrid::new(&mut self.filetree_pane_state, |pane, file_tree| {
                file_tree.view(pane).map(PaneState::FileTreeMessage).into()
            });
        let text_buffer_pane_grid =
            PaneGrid::new(&mut self.textbuffer_pane_state, |pane, text_buffer| {
                text_buffer.view().map(PaneState::TextBufferMessage).into()
            });
        Column::new()
            .push(file_tree_pane_grid)
            .push(text_buffer_pane_grid)
            .into()
    }
}

/// A simple styling library taken from the exaples. <br>
///  Provides styling for **Button, Pane**.
mod style {
    use iced::button;
    use iced::{pane_grid, Background, Color, Vector};

    /// Generic style of a Notal button.
    pub enum Button {
        Primary,
        Secondary,
    }

    pub enum Pane {
        TextBufferPane,
        FileTreePane,
    }
    impl pane_grid::StyleSheet for Pane {
        fn hovered_split(&self) -> Option<pane_grid::Line> {
            self.hovered_split()
        }
        fn picked_split(&self) -> Option<pane_grid::Line> {
            match self {
                Pane::TextBufferPane => Some(pane_grid::Line {
                    color: Color::WHITE,
                    width: 800.0,
                }),
                Pane::FileTreePane => Some(pane_grid::Line {
                    color: Color::from_rgb8(163, 213, 172),
                    width: 200.0,
                }),
            }
        }
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                })),
                border_radius: 12.0,
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
