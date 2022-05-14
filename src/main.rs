use iced::{
    button,
    window::{self, Mode},
    Application, Button, Column, Container, Element, Settings, Text,
};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use std::path::{Path, PathBuf};
pub fn main() -> iced::Result {
    NotalApp::run(Settings {
        id: Some(String::from("notal_app")),
        window: iced::window::Settings::default(),
        flags: (),
        default_font: None,
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
    Loaded(TextBuffer), //* Bu noktada, input bufferımız açılmış olmalı. */
}

/// Mesajlar, update fonksiyonlarında kontrol edilir ve buna göre işlem yapılır
/// Mesajlar, ui elementlerinin içerisinde kullanılabilir. butona basıldığında şu mesajı yolla gibi.
#[derive(Debug, Clone)]
pub enum Message {
    OpenBuffer, //* Burada, uygulamada, yüklü bir dosya varmı yokmu o belli olacak., Loaderror, bize FileDialogState'i vermeli.
    CloseBuffer,
    OpenMenu,
    MainMenuMessage(MainMenuMessage),
    TextBufferMessage(TextBufferMessage),
}
/// Genel mimari;
/// eğer uygulamada önceden bir dosya açılmışsa, o zaman dosya aç kısmını geç.
/// Eğer açılmamışsa, ana menüyü göster. Ana menüde ise, dosya açma tuşu, ve yeni dosya açma tuşu ile beraber bir logo bulunmalı.
/// Menü için yapılması gerekenler. Root'un Update kısmında, Eğer applikasyon loading'te ise, Loaded yap. bunu yaparken iki seçenek var. ya uygulama yüklenir yada yüklenmez.
/// Yüklenmez ise, hata verir ve menüyü açma güncellemesi yapar.
/// Root'un view metodunda, gelen mesaj, Error içeriyorsa, bizim FileDialogState'in implementasyonunu yükle.
///
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
                    menu.update(main_msg);
                }
                _ => (),
            },
            NotalApp::Loaded(text_buffer) => match message {
                Message::TextBufferMessage(txt_msg) => {
                    text_buffer.update(txt_msg);
                }
                _ => (),
            },
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
                .into(),
            NotalApp::Loaded(text_buffer) => {
                Container::new(text_buffer.view().map(Message::TextBufferMessage)).into()
            }
        }
    }
}

///FileDialogState for **Elm - *State* architechture**
#[derive(Debug, Clone)]
pub struct MainMenu {
    file_dialog_opened: bool,
    open_file_button: button::State,
    new_file_button: button::State,
    file_path: PathBuf,
}
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
        Column::new()
            .push(
                Button::new(&mut self.open_file_button, Text::new("Dosya Aç"))
                    .on_press(MainMenuMessage::OpenFilePressed),
            )
            .push(
                Button::new(&mut self.new_file_button, Text::new("Yeni Dosya Aç."))
                    .on_press(MainMenuMessage::NewFilePressed),
            )
            .into()
    }
}

/// Checks if the file given is in the true format.
#[derive(Debug, Clone)]
enum LoadError {
    FileError,
    FormatError,
}

/// Commands for the FileDialog Opener.
enum FileDialogCommands {
    SHOW_SAVE_FILE_DIALOG,
    SHOW_OPEN_FILE_DIALOG,
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
                .add_filter("Text Dosyası", &["txt"])
                .add_filter("Markdown Dosyaları", &["md"])
                .show_open_single_file()
                .unwrap();
            match path {
                Some(path) => Ok(path),
                None => Err(FileDialogError::FileNotFoundError),
            }
        }
        FileDialogCommands::SHOW_SAVE_FILE_DIALOG => {
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
    fn new() -> Self {
        Self {
            input_line: "".to_string(),
            file_contents: "".to_string(),
            file_path: PathBuf::new(),
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
        Column::new().push(Text::new("Empty Buffer")).into()
    }
}
