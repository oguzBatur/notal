use beryllium::{self, *};
use gl33::global_loader::*;
use gl33::*;
// Local crates.
mod inputs;
mod markdown;
//TODO küçük soyutlamalar ile ilerle.
//TODO girdi ve çıktı mantığını çöz ve bunda istediğini aldıktan sonra opengl'e geç.
const BG_COLOR: (f32, f32, f32, f32) = (184.0 / 255.0, 213.0 / 255.0, 238.0 / 255.0, 1.0);
// Main is here.
fn main() {
    inputs::take_continous_input();
    // SDL'i başlat
    let sdl = SDL::init(InitFlags::Video).expect("SDL baslatilamadi");
    let pencere = create_window(&sdl);
    unsafe {
        load_global_gl(|f_name| pencere.get_proc_address(f_name));
        draw(&pencere);
    }
    'main_loop: loop {
        while let Some(event) = sdl.poll_events().and_then(Result::ok) {
            match event {
                Event::Quit(_) => break 'main_loop,
                _ => (),
            }
            unsafe {
                draw(&pencere);
            }
        }
    }
} // Main ends here.

// Çiz.
unsafe fn draw(window: &GlWindow) {
    glClearColor(BG_COLOR.0, BG_COLOR.1, BG_COLOR.2, BG_COLOR.3);
    glClear(GL_COLOR_BUFFER_BIT);
    window.swap_window();
}

fn create_window(sdl: &SDL) -> GlWindow {
    sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core)
        .unwrap();
    sdl.create_gl_window(
        "Notal",
        WindowPosition::Centered,
        800,
        600,
        WindowFlags::Shown,
    )
    .expect("Pencere Yaratilamadi")
}

// Create a shader.
unsafe fn _create_shaders(vertex_shader_source: &str, fragment_shader_source: &str) -> u32 {
    // Vertex Shader..
    let vertex_shader = glCreateShader(GL_VERTEX_SHADER);
    assert_ne!(vertex_shader, 0);
    glShaderSource(
        vertex_shader,
        1,
        &(vertex_shader_source.as_bytes().as_ptr().cast()),
        &(vertex_shader_source.len().try_into().unwrap()),
    );
    glCompileShader(vertex_shader);

    // Fragment Shader..
    let fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
    assert_ne!(fragment_shader, 0);
    glShaderSource(
        fragment_shader,
        1,
        &(fragment_shader_source.as_bytes().as_ptr().cast()),
        &(fragment_shader_source.len().try_into().unwrap()),
    );
    glCompileShader(fragment_shader);

    let shader_program = glCreateProgram();
    assert_ne!(shader_program, 0);
    glAttachShader(shader_program, vertex_shader);
    glAttachShader(shader_program, fragment_shader);
    glDeleteShader(vertex_shader);
    glDeleteShader(fragment_shader);
    shader_program
}
