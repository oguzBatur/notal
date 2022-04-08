// SDL kütüphanesi - SDL library.
use beryllium::*;
// OpenGl kütüphanesi - OpenGl library
use core::mem::*;
use ogl33::*;

fn main() {
    // SDL'i aç. - Turn on SDL.
    let sdl = init_sdl();
    // Pencere Yarat. - Create Window.
    let window = create_window_default(&sdl);
    // OpenGL'i aç. - open OpenGL.
    unsafe {
        load_gl_with(|f_name| window.get_proc_address(f_name));
        glClearColor(0.2, 0.3, 0.3, 1.0);
        let mut vao = 0;
        glGenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        let mut vbo = 0;
        glGenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);
        glBindBuffer(GL_ARRAY_BUFFER, vbo);
        type Vertex = [f32; 3];
        const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];
        glBufferData(
            GL_ARRAY_BUFFER,
            size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
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
    }
}

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
            800,
            600,
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
    }
    win
}
