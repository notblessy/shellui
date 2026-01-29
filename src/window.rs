//! Window and event loop: winit + softbuffer.

use std::num::NonZeroU32;
use std::rc::Rc;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowAttributes;

use crate::app::{Scene, WindowConfiguration};
use crate::layout::{layout, Limits};
use crate::render::Renderer;
use crate::View;

/// Wraps a winit Window in Rc so we can share it with softbuffer::Surface (which requires Clone).
#[derive(Clone)]
struct WindowRef(Rc<winit::window::Window>);

impl HasWindowHandle for WindowRef {
    fn window_handle(&self) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        self.0.window_handle()
    }
}

impl HasDisplayHandle for WindowRef {
    fn display_handle(&self) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        self.0.display_handle()
    }
}

/// Runs the shellui app: opens a window and runs the event loop.
/// The view is built once; layout and render run on each frame (e.g. on resize/redraw).
pub fn run(view: impl FnOnce() -> View) {
    let view = view();
    let mut renderer = Renderer::new();
    let _ = renderer.load_default_font();

    let event_loop = winit::event_loop::EventLoop::new().expect("event loop");
    let display_handle = event_loop.owned_display_handle();
    let context = softbuffer::Context::new(display_handle).expect("softbuffer context");
    let mut app = InternalApp {
        context,
        window: None,
        surface: None,
        view,
        renderer,
        config: WindowConfiguration::default(),
    };
    let _ = event_loop.run_app(&mut app);
}

/// Runs a scene-based shellui app.
pub fn run_scene(scene: Scene) {
    match scene {
        Scene::WindowGroup { content, config } => {
            let view = content();
            let mut renderer = Renderer::new();
            let _ = renderer.load_default_font();

            let event_loop = winit::event_loop::EventLoop::new().expect("event loop");
            let display_handle = event_loop.owned_display_handle();
            let context = softbuffer::Context::new(display_handle).expect("softbuffer context");
            let mut app = InternalApp {
                context,
                window: None,
                surface: None,
                view,
                renderer,
                config,
            };
            let _ = event_loop.run_app(&mut app);
        }
    }
}

struct InternalApp<D> {
    context: softbuffer::Context<D>,
    window: Option<WindowRef>,
    surface: Option<softbuffer::Surface<D, WindowRef>>,
    view: View,
    renderer: Renderer,
    config: WindowConfiguration,
}

impl<D> ApplicationHandler for InternalApp<D>
where
    D: HasDisplayHandle,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let mut window_attrs = WindowAttributes::default()
            .with_title(&self.config.title)
            .with_inner_size(winit::dpi::LogicalSize::new(self.config.size.0, self.config.size.1))
            .with_resizable(self.config.resizable);

        if let Some((min_w, min_h)) = self.config.min_size {
            window_attrs = window_attrs.with_min_inner_size(winit::dpi::LogicalSize::new(min_w, min_h));
        }

        if let Some((max_w, max_h)) = self.config.max_size {
            window_attrs = window_attrs.with_max_inner_size(winit::dpi::LogicalSize::new(max_w, max_h));
        }

        if self.config.fullscreen {
            window_attrs = window_attrs.with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
        }

        let window = event_loop
            .create_window(window_attrs)
            .expect("create window");

        let window_ref = WindowRef(Rc::new(window));
        let surface = softbuffer::Surface::new(&self.context, window_ref.clone())
            .expect("softbuffer surface");
        self.window = Some(window_ref);
        self.surface = Some(surface);

        self.draw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(ref window) = self.window {
                    window.0.request_redraw();
                }
                if let (Some(ref mut surface), Some(w), Some(h)) = (
                    &mut self.surface,
                    NonZeroU32::new(size.width),
                    NonZeroU32::new(size.height),
                ) {
                    let _ = surface.resize(w, h);
                }
            }
            WindowEvent::RedrawRequested => {
                self.draw();
            }
            _ => {}
        }
    }
}

impl<D> InternalApp<D>
where
    D: HasDisplayHandle,
{
    fn draw(&mut self) {
        let Some(ref mut surface) = self.surface else { return };
        let Some(ref window) = self.window else { return };
        let size = window.0.inner_size();
        let (width, height) = (size.width, size.height);
        if width == 0 || height == 0 {
            return;
        }
        if let (Some(w), Some(h)) = (NonZeroU32::new(width), NonZeroU32::new(height)) {
            let _ = surface.resize(w, h);
        }
        let Ok(mut buffer) = surface.buffer_mut() else { return };
        let pixels: &mut [u32] = &mut *buffer;
        let limits = Limits::loose(width as f32, height as f32);
        let layout_root = layout(&self.view, limits, &self.renderer);
        let offset_x = (width as f32 - layout_root.bounds.width) / 2.0;
        let offset_y = (height as f32 - layout_root.bounds.height) / 2.0;
        let background = 0x00_EE_EE_EEu32; // light gray
        self.renderer.draw(
            &self.view,
            &layout_root,
            pixels,
            width,
            height,
            background,
            offset_x,
            offset_y,
        );
        let _ = buffer.present();
    }
}
