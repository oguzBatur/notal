use env_logger;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

// create and run a window.
pub fn run() {
    env_logger::init();
    let event_loop = EventLoop::new(); // Creata a new EventLoop for the window.
    let window = WindowBuilder::new().build(&event_loop).unwrap(); // Create a new window, and attach the event loop into it.
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}

// Durum yapısı.
struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
}

impl State {
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // Ekran kartı ile uğraşmak adına bir instance yaratıyoruz.
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        // Burada, backends'i all yaparak, üretilecek uygulamanın cross-platfoırm olduğunu belirtiyoruz.
        let instance = wgpu::Instance::new(wgpu::Backends::all);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size,
        }
    }

    // Pencereyi boyutlandır.
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        todo!();
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!();
    }

    fn update(&mut self) {
        todo!();
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!();
    }
}
