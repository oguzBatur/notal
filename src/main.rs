// SDL kütüphanesi - SDL library.
use beryllium::*;
// OpenGl kütüphanesi - OpenGl library
use core::mem::*;
use ogl33::*;

//TODO Pencerenin içerisine arkaplan grafiği çiz.
//TODO basit bir UI yarat.

// Sürekli Değerler - Constant Variables.
const DEFAULT_WINDOW_SIZE: [u32; 2] = [800, 600];
// Types
#[warn(dead_code)]
type Vertex = [f32; 3]; // tipik bir vertex. -  a typical vertex.
type Rgba = (f32, f32, f32, f32); // KMYA renk paleti - RGBA palette type
#[warn(dead_code)]
enum Color {
    RGBA(Rgba),
}
// Main is here.
fn main() {
    // SDL'i aç. - Turn on SDL.
    let sdl = init_sdl();
    // Pencere Yarat. - Create Window.
    let window = create_window_default(&sdl);

    unsafe {
        // OpenGL'i aç. - open OpenGL.
        load_gl_with(|f_name| window.get_proc_address(f_name));
        // Create simple graphic.
        const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

        const COLOR: Rgba = (0.3, 0.4, 0.5, 1.0);
        create_vertex_graphics(VERTICES, COLOR);
    }
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
    // Program çalışmayı durdurdu probleminin oluşmasını, genel loop ile çözüyoruz. - We prevent program has stopped working issue with a main loop
    'main_loop: loop {
        while let Some(event) = sdl.poll_events().and_then(Result::ok) {
            match event {
                Event::Quit(_) => break 'main_loop,
                _ => (),
            }
        }
        win.swap_window();
    }
    win
}

// Create graphics.
unsafe fn create_vertex_graphics(vertex: [Vertex; 3], rgba: Rgba) {
    glClearColor(rgba.0, rgba.1, rgba.2, rgba.3);
    let mut vertical_array_object = 0;
    glGenVertexArrays(1, &mut vertical_array_object);
    assert_ne!(vertical_array_object, 0);
    let mut vertical_binding_object = 0;
    glGenBuffers(1, &mut vertical_binding_object);
    assert_ne!(vertical_binding_object, 0);
    glBindBuffer(GL_ARRAY_BUFFER, vertical_binding_object);
    glBufferData(
        GL_ARRAY_BUFFER,
        size_of_val(&vertex) as isize,
        vertex.as_ptr().cast(),
        GL_STATIC_DRAW,
    );
    glVertexAttribPointer(
        0,
        3,
        GL_FLOAT,
        GL_FALSE,
        size_of::<Vertex>().try_into().unwrap(),
        0 as *const _,
    );
    glEnableVertexAttribArray(0);
    glViewport(
        0,
        0,
        DEFAULT_WINDOW_SIZE[0] as i32,
        DEFAULT_WINDOW_SIZE[1] as i32,
    );
}
