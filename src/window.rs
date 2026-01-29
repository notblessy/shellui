//! Window and event loop: winit + softbuffer.

use std::num::NonZeroU32;
use std::rc::Rc;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::application::ApplicationHandler;
use winit::event::{WindowEvent, MouseButton, ElementState};
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowAttributes;

use crate::app::{ContentPosition, ContentSizing, Scene, WindowConfiguration};
use crate::layout::{layout, Limits, Node, Rectangle};
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
pub fn run(view_fn: impl Fn() -> View + 'static) {
    let mut renderer = Renderer::new();
    let _ = renderer.load_default_font();

    let event_loop = winit::event_loop::EventLoop::new().expect("event loop");
    let display_handle = event_loop.owned_display_handle();
    let context = softbuffer::Context::new(display_handle).expect("softbuffer context");
    let mut app = InternalApp {
        context,
        window: None,
        surface: None,
        content_fn: view_fn,
        renderer,
        config: WindowConfiguration::default(),
        layout_root: None,
        offset_x: 0.0,
        offset_y: 0.0,
        cursor_pos: (0.0, 0.0),
    };
    let _ = event_loop.run_app(&mut app);
}

/// Runs a scene-based shellui app.
pub fn run_scene(scene: Scene) {
    match scene {
        Scene::WindowGroup { content, config } => {
            let mut renderer = Renderer::new();
            let _ = renderer.load_default_font();

            let event_loop = winit::event_loop::EventLoop::new().expect("event loop");
            let display_handle = event_loop.owned_display_handle();
            let context = softbuffer::Context::new(display_handle).expect("softbuffer context");
            let mut app = InternalApp {
                context,
                window: None,
                surface: None,
                content_fn: content,
                renderer,
                config,
                layout_root: None,
                offset_x: 0.0,
                offset_y: 0.0,
                cursor_pos: (0.0, 0.0),
            };
            let _ = event_loop.run_app(&mut app);
        }
    }
}

struct InternalApp<D, F> {
    context: softbuffer::Context<D>,
    window: Option<WindowRef>,
    surface: Option<softbuffer::Surface<D, WindowRef>>,
    content_fn: F,
    renderer: Renderer,
    config: WindowConfiguration,
    layout_root: Option<Node>,
    offset_x: f32,
    offset_y: f32,
    cursor_pos: (f32, f32),
}

impl<D, F> ApplicationHandler for InternalApp<D, F>
where
    D: HasDisplayHandle,
    F: Fn() -> View,
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
            WindowEvent::MouseInput { button: MouseButton::Left, state: ElementState::Pressed, .. } => {
                // Handle mouse clicks
                self.handle_click(self.cursor_pos.0, self.cursor_pos.1);
            }
            WindowEvent::CursorMoved { position, .. } => {
                // Store mouse position for click testing
                self.cursor_pos = (position.x as f32, position.y as f32);
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

impl<D, F> InternalApp<D, F>
where
    D: HasDisplayHandle,
    F: Fn() -> View,
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
        
        // Rebuild view tree on each draw for reactivity
        let view = (self.content_fn)();
        
        // Determine content limits based on sizing mode
        let (content_limits, offset_x, offset_y) = match self.config.content_sizing {
            ContentSizing::Auto => {
                // Use loose limits and position based on content_position
                let limits = Limits::loose(width as f32, height as f32);
                let layout_root = layout(&view, limits, &self.renderer);
                let (offset_x, offset_y) = InternalApp::<D, F>::calculate_content_offset(
                    &self.config,
                    layout_root.bounds.width,
                    layout_root.bounds.height,
                    width as f32,
                    height as f32
                );
                (limits, offset_x, offset_y)
            },
            ContentSizing::FillWindow => {
                // Use exact window size limits
                let limits = Limits {
                    min_width: width as f32,
                    min_height: height as f32,
                    max_width: width as f32,
                    max_height: height as f32,
                };
                (limits, 0.0, 0.0)
            },
            ContentSizing::Fixed(w, h) => {
                // Use fixed size limits and position based on content_position
                let limits = Limits {
                    min_width: w,
                    min_height: h,
                    max_width: w,
                    max_height: h,
                };
                let (offset_x, offset_y) = InternalApp::<D, F>::calculate_content_offset(&self.config, w, h, width as f32, height as f32);
                (limits, offset_x, offset_y)
            },
            ContentSizing::Minimum(min_w, min_h) => {
                // Use minimum size but allow expansion
                let limits = Limits {
                    min_width: min_w,
                    min_height: min_h,
                    max_width: width as f32,
                    max_height: height as f32,
                };
                let layout_root = layout(&view, limits, &self.renderer);
                let (offset_x, offset_y) = InternalApp::<D, F>::calculate_content_offset(
                    &self.config,
                    layout_root.bounds.width,
                    layout_root.bounds.height,
                    width as f32,
                    height as f32
                );
                (limits, offset_x, offset_y)
            },
        };
        
        let layout_root = layout(&view, content_limits, &self.renderer);
        
        // Store layout and offset for click testing
        self.layout_root = Some(layout_root.clone());
        self.offset_x = offset_x;
        self.offset_y = offset_y;
        
        let background = 0x00_EE_EE_EEu32; // light gray
        self.renderer.draw(
            &view,
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

    fn calculate_content_offset(
        config: &WindowConfiguration,
        content_width: f32,
        content_height: f32,
        window_width: f32,
        window_height: f32,
    ) -> (f32, f32) {
        let extra_width = window_width - content_width;
        let extra_height = window_height - content_height;

        match config.content_position {
            ContentPosition::Leading => (0.0, 0.0), // Top-left
            ContentPosition::Center => (extra_width / 2.0, extra_height / 2.0), // Center
            ContentPosition::Trailing => (extra_width, extra_height), // Bottom-right
            ContentPosition::TopCenter => (extra_width / 2.0, 0.0), // Top-center
            ContentPosition::BottomCenter => (extra_width / 2.0, extra_height), // Bottom-center
        }
    }

    fn handle_click(&self, x: f32, y: f32) {
        if let Some(ref layout_root) = self.layout_root {
            let view = (self.content_fn)();
            self.test_click(&view, layout_root, x, y, self.offset_x, self.offset_y);
        }
    }

    fn test_click(&self, view: &View, node: &Node, x: f32, y: f32, offset_x: f32, offset_y: f32) {
        // Adjust coordinates for content offset
        let local_x = x - offset_x;
        let local_y = y - offset_y;
        
        // Test if click is within this node's bounds
        if local_x >= node.bounds.x &&
           local_x <= node.bounds.x + node.bounds.width &&
           local_y >= node.bounds.y &&
           local_y <= node.bounds.y + node.bounds.height {
            
            // Check if this is a button
            if let View::Button(button) = view {
                if let Some(ref callback) = button.on_click {
                    callback();
                    // Request a redraw after button click to update UI
                    if let Some(ref window) = self.window {
                        window.0.request_redraw();
                    }
                    return; // Found a button, execute callback and stop
                }
            }
            
            // Recursively check children
            match view {
                View::VStack(vstack) => {
                    for (child_view, child_node) in vstack.children.iter().zip(node.children.iter()) {
                        self.test_click(child_view, child_node, x, y, offset_x + node.bounds.x, offset_y + node.bounds.y);
                    }
                }
                View::HStack(hstack) => {
                    for (child_view, child_node) in hstack.children.iter().zip(node.children.iter()) {
                        self.test_click(child_view, child_node, x, y, offset_x + node.bounds.x, offset_y + node.bounds.y);
                    }
                }
                _ => {
                    // For other views, check children if any
                    for child_node in &node.children {
                        // This is a simplified version - in a real implementation,
                        // we'd need to track which child view corresponds to which node
                    }
                }
            }
        }
    }
}
