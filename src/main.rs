// SDL kütüphanesi - SDL library.
use beryllium::*;
// OpenGl kütüphanesi - OpenGl library
use core::mem::*;
use gl33::global_loader::*;
use gl33::*;

// Local crates.
mod inputs;
mod markdown;
//TODO küçük soyutlamalar ile ilerle.

// Sürekli Değerler - Constant Variables.
const DEFAULT_WINDOW_SIZE: [u32; 2] = [800, 600];
const VERT_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;
      void main() {
        gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
          }
    "#;
const FRAG_SHADER: &str = r#"#version 330 core
  out vec4 final_color;
    
      void main() {
        final_color = vec4(1.0, 0.5, 0.2, 1.0);

          }
    "#;
// Types
#[warn(dead_code)]
type Vertex = [f32; 3]; // tipik bir vertex. -  a typical vertex.
type Rgba = (f32, f32, f32, f32); // KMYA renk paleti - RGBA palette type

// ViewPort Struct
struct ViewPort {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

//RGB renk enum.
enum Color {
    RGBA(Rgba),
}

// Main is here.
fn main() {
    // SDL'i aç. - Turn on SDL.
    let sdl = init_sdl();
    // Pencere Yarat. - Create Window.
    let _window = create_window_default(&sdl);
} // Main ends here.

fn init_sdl() -> SDL {
    SDL::init(InitFlags::Everything).expect("SDL baslatilamadi")
}

// Default ayarlar ile pencere yarat. - Create a window with default settings.
fn create_window_default(sdl: &SDL) -> GlWindow {
    // Özellik belirle. - Set attribute.
    sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
    // Cihaz'da bulunan GL özellkillerinin hepsi. - All the GL features the OS spec has.
    sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core)
        .unwrap();

    // MacOS için özel ayarlar. - Special settings for MacOS.
    #[cfg(target_os = "macos")]
    {
        sdl.gl_set_attribute(SdlGlAttr::Flags, ContextFlag::ForwardCompatible)
            .unwrap();
    }

    let win = sdl
        .create_gl_window(
            "Notal",
            WindowPosition::Centered,
            DEFAULT_WINDOW_SIZE[0],
            DEFAULT_WINDOW_SIZE[1],
            WindowFlags::Shown,
        )
        .expect("pencere yaratilamadi");
    // Renkler max 1.0 olması lazım, renk uygularken 255' e bölersek rengi alıyoruz ...
    const BG_COLOR: Rgba = (184.0 / 255.0, 213.0 / 255.0, 238.0 / 255.0, 1.0);

    unsafe {
        let vertices: [Vertex; 6] = [
            [-0.5, -0.5 * f32::sqrt(3.0) / 3.0, 0.0],
            [0.5, -0.5 * f32::sqrt(3.0) / 3.0, 0.0],
            [0.0, 0.5 * f32::sqrt(3.0) * 2.0 / 3.0, 0.0],
            [-0.5 / 2.0, 0.5 * f32::sqrt(3.0) / 6.0, 0.0],
            [0.5 / 2.0, 0.5 * f32::sqrt(3.0) / 6.0, 0.0],
            [0.0, -0.5 * f32::sqrt(3.0) / 6.0, 0.0],
        ];
        const VIEWPORT: ViewPort = ViewPort {
            x: 0,
            y: 0,
            width: 800,
            height: 600,
        };
        // Arkaplan buffer'ı yarat. - Create a back buffer.
        create_opengl_buffer(BG_COLOR, VIEWPORT);
        // Vsync'i aç. - init Vsync.
        win.set_swap_interval(SwapInterval::Vsync);
        win.swap_window();
    }
    'main_loop: loop {
        while let Some(event) = sdl.poll_events().and_then(Result::ok) {
            match event {
                Event::Quit(_) => break 'main_loop,
                _ => (),
            }

            // Sürekli pencere bufferını çalıştırmamız lazım, aynı zamanda sürekli olarak çizim eyleminin gerçekleşmesi lazım.
            unsafe {
                glClearColor(BG_COLOR.0, BG_COLOR.1, BG_COLOR.2, BG_COLOR.3);
                glClear(GL_COLOR_BUFFER_BIT);
                let program = create_shaders(VERT_SHADER, FRAG_SHADER);
                glUseProgram(program);
                glBindVertexArray(gl_elements_tuple.0 as u32);
                glDrawElements(GL_TRIANGLES, 9, GL_UNSIGNED_INT, 0 as *const _);
                win.swap_window();
            }
        }
    }
    win
}

// Grafik Yarat. -  Create graphics.
unsafe fn create_opengl_buffer(rgba: Rgba, viewport: ViewPort) {
    // viewport of openGL
    glViewport(viewport.x, viewport.y, viewport.width, viewport.height);
    // Arka buffer (Pencere) - The back buffer
    glClearColor(rgba.0, rgba.1, rgba.2, rgba.3);
    // Ön buffer (Pencere) - Front Buffer
    glClear(GL_COLOR_BUFFER_BIT);
}

unsafe fn create_triangle(vertex: [Vertex; 6]) -> u32 {
    // İlk işlem, vertex array object (VAO) yarat. - First, create a vertex array object. (VAO)
    let mut vao = 0;
    glGenVertexArrays(1, &mut vao);
    assert_ne!(vao, 0);
    glBindVertexArray(vao);

    // İkinci işlem, vertex bound object(VBO) yarat. - Second, create a vertex bound object (VBO)
    let mut vbo = 0;
    glGenBuffers(1, &mut vbo);
    assert_ne!(vbo, 0);
    glBindBuffer(GL_ARRAY_BUFFER, vbo);

    // In order to create more complex shapes, we must use the Index Buffer Option
    let indices: [[i32; 3]; 3] = [[0, 3, 5], [3, 2, 4], [5, 4, 1]];
    let mut ebo = 0;
    glGenBuffers(1, &mut ebo);
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);
    glBufferData(
        GL_ELEMENT_ARRAY_BUFFER,
        size_of_val(&indices) as isize,
        indices.as_ptr().cast(),
        GL_STATIC_DRAW,
    );

    glBufferData(
        GL_ARRAY_BUFFER,
        size_of_val(&vertex) as isize,
        vertex.as_ptr().cast(),
        GL_STATIC_DRAW,
    );

    glVertexAttribPointer(
        0,
        6,
        GL_FLOAT,
        0,
        size_of::<Vertex>().try_into().unwrap(),
        0 as *const _,
    );
    glEnableVertexAttribArray(0);

    vao
}

// Create a shader.
unsafe fn create_shaders(vertex_shader_source: &str, fragment_shader_source: &str) -> u32 {
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
