// Local crates.
mod greeter_ui;
mod inputs;
mod markdown;
use greeter_ui::notal_greeter_ui;
mod windows;
use druid::{
    text::RichText, AppDelegate, AppLauncher, Data, Env, Lens, LocalizedString, WindowDesc,
};
fn main() {
    let main_window = WindowDesc::new(notal_greeter_ui)
        .title(WINDOW_TITLE)
        .window_size((800.0, 600.0));
    //* Başlangıç durumu */
    let initial_state = NotalAppState {
        file_state: FileState{
            current_file_contents: "".to_string(),
            file_modified: false,
            file_opened: false,
            new_file_contents: "".to_string(),
            raw: "".to_string(),
        }

    } 
    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .use_simple_logger()
        .launch(initial_state)
        .expect("Failed to launch the app");
}

//* Uygulama Başlığı. */
const WINDOW_TITLE: LocalizedString<NotalAppState> = LocalizedString::new("Notal");

//* Uygulamamızın içerisindeki değerleri manipüle etmek ve görmek için, ortak bir struct yaratıyoruz. */
#[derive(Clone, Data, Lens)]
struct NotalAppState {
    file_state: FileState,
}

#[derive(Clone, Data, Lens)]
struct FileState {
    file_modified: bool,
    current_file_contents: String,
    new_file_contents: String,
    file_opened: bool,
    raw: String,
    rendered: RichText,
}

//* Uygulama içindeki pencere eventlerini delegate denilen struct üzerinde, AppDelegate implementasyonu ile halledicez. */
struct Delegate;
impl AppDelegate<NotalAppState> for Delegate {
    //* Traitleri structlara implemente ederken, Traitlerin kendi içerilerinde barındırdıkları, fonksiyonları structa özel modifiye edebiliyoruz. */
    fn command(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        target: druid::Target,
        cmd: &druid::Command,
        data: &mut NotalAppState,
        env: &Env,
    ) -> druid::Handled {
        //* Eğer, SAVE_FILE_AS komutu gelirse, ve bir hata olursa printle, yoksa ise druid::Handled::Yes enumu ile kontrol et. */
        if let Some(file_info) = cmd.get(druid::commands::SAVE_FILE_AS) {
            if let Err(e) = std::fs::write(file_info.path(), &data.file_state.current_file_contents)
            {
                println!("Error Writing File: {}", e);
            }
            return druid::Handled::Yes;
        }
        if let Some(file_info) = cmd.get(druid::commands::OPEN_FILE) {
            match std::fs::read_to_string(file_info.path()) {
                Ok(s) => {
                    println!("File has been opened");
                    //TODO şu küçük sıkıntıyı çöz
                    *data.file_state.new_file_contents = s.to_owned();
                }
                Err(e) => {
                    println!("Error opening file {}", e);
                }
            }
            return druid::Handled::Yes;
        }
        druid::Handled::No
    }
}
