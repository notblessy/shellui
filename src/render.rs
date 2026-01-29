//! Render view tree + layout to a pixel buffer (text and background).

use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};
use std::path::Path;
use std::sync::{RwLock, OnceLock};
use std::collections::{HashMap, HashSet};
use std::borrow::Cow;

use crate::layout::{Node, Rectangle, Size, TextMeasurer};
use crate::view::View;
use crate::core::background::{Background, Color};
use crate::core::renderer::{Quad, Renderer as RendererTrait};
use crate::core::transformation::Transformation;

/// Default font bundled in the crate (Roboto Regular).
pub static DEFAULT_FONT: &[u8] =
    include_bytes!("../assets/fonts/Roboto-Regular.ttf");

/// Default text size in logical pixels.
pub const DEFAULT_FONT_SIZE: f32 = 16.0;

/// Returns the global [`FontSystem`].
fn font_system() -> &'static RwLock<FontSystem> {
    static FONT_SYSTEM: OnceLock<RwLock<FontSystem>> = OnceLock::new();

    FONT_SYSTEM.get_or_init(|| {
        RwLock::new(FontSystem::new())
    })
}

/// Renders the view tree using the layout tree. Handles font loading, text measurement, and drawing.
pub struct Renderer {
    loaded_fonts: HashSet<usize>,
    glyph_cache: GlyphCache,
}

impl Renderer {
    pub fn new() -> Self {
        Self { 
            loaded_fonts: HashSet::new(),
            glyph_cache: GlyphCache::new(),
        }
    }

    /// Load the bundled default font (Roboto Regular). Text works out of the box without adding a TTF.
    pub fn load_default_font(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.load_font_bytes(DEFAULT_FONT.into())
    }

    /// Try to load font from a path (e.g. "assets/font.ttf" or env FONT_PATH).
    pub fn load_font(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let bytes = std::fs::read(path.as_ref())?;
        self.load_font_bytes(bytes.into())
    }

    /// Load font from bytes.
    fn load_font_bytes(&mut self, bytes: Cow<'static, [u8]>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Cow::Borrowed(bytes) = bytes {
            let address = bytes.as_ptr() as usize;
            if !self.loaded_fonts.insert(address) {
                return Ok(());
            }
        }

        let mut font_system = font_system().write().unwrap();
        font_system.db_mut().load_font_source(cosmic_text::fontdb::Source::Binary(std::sync::Arc::new(
            bytes.into_owned(),
        )));
        Ok(())
    }

    /// Measure text for layout using cosmic-text.
    pub fn measure_text(&self, text: &str, font_size: f32) -> Size {
        let mut font_system = font_system().write().unwrap();
        let metrics = Metrics::new(font_size, font_size * 1.2);
        let mut buffer = Buffer::new(&mut font_system, metrics);
        buffer.set_size(&mut font_system, Some(f32::MAX), Some(f32::MAX));
        buffer.set_text(&mut font_system, text, &Attrs::new(), Shaping::Advanced, None);
        
        let (width, height) = buffer
            .layout_runs()
            .fold((0.0, 0.0), |(width, height), run| {
                (run.line_w.max(width), height + run.line_height)
            });
            
        Size::new(width.max(1.0), height.max(font_size))
    }

    /// Draw the view tree into a buffer (0x00RRGGBB u32, row-major, width * height).
    /// Background is cleared to background_color. Offset (e.g. for centering) is added to all positions.
    pub fn draw(
        &mut self,
        view: &View,
        layout_root: &Node,
        buffer: &mut [u32],
        width: u32,
        height: u32,
        background_color: u32,
        offset_x: f32,
        offset_y: f32,
    ) {
        if (width as usize) * (height as usize) != buffer.len() {
            return;
        }
        for p in buffer.iter_mut() {
            *p = background_color;
        }
        let rect = Rectangle::new(offset_x, offset_y, width as f32, height as f32);
        self.draw_view(view, layout_root, buffer, width, height, rect);
    }

    fn draw_view(
        &mut self,
        view: &View,
        node: &Node,
        buffer: &mut [u32],
        width: u32,
        height: u32,
        parent_rect: Rectangle,
    ) {
        let bounds = node.bounds;
        let abs_rect = Rectangle::new(
            parent_rect.x + bounds.x,
            parent_rect.y + bounds.y,
            bounds.width,
            bounds.height,
        );

        match view {
            View::Text(t) => {
                let font_size = t.size.unwrap_or(DEFAULT_FONT_SIZE);
                let text_color = t.color.unwrap_or(Color::new(0.0, 0.0, 0.0, 1.0)); // Default to black
                self.draw_text(&t.string, abs_rect, font_size, text_color, buffer, width, height);
            }
            View::VStack(v) => {
                // Draw background if present
                if let Some(background) = &v.background {
                    self.draw_background(background, abs_rect, buffer, width, height);
                }
                // Draw children
                for (child_view, child_node) in v.children.iter().zip(node.children.iter()) {
                    self.draw_view(child_view, child_node, buffer, width, height, abs_rect);
                }
            }
            View::HStack(h) => {
                // Draw background if present
                if let Some(background) = &h.background {
                    self.draw_background(background, abs_rect, buffer, width, height);
                }
                // Draw children
                for (child_view, child_node) in h.children.iter().zip(node.children.iter()) {
                    self.draw_view(child_view, child_node, buffer, width, height, abs_rect);
                }
            }
        }
    }

    fn draw_text(
        &mut self,
        text: &str,
        rect: Rectangle,
        font_size: f32,
        color: Color,
        buffer: &mut [u32],
        buf_width: u32,
        buf_height: u32,
    ) {
        let mut font_system = font_system().write().unwrap();
        let metrics = Metrics::new(font_size, font_size * 1.2);
        let mut text_buffer = Buffer::new(&mut font_system, metrics);
        text_buffer.set_size(&mut font_system, Some(rect.width), Some(rect.height));
        text_buffer.set_text(&mut font_system, text, &Attrs::new(), Shaping::Advanced, None);
        
        let mut swash = SwashCache::new();
        let color_rgb = [
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8, 
            (color.b * 255.0) as u8
        ];
        
        for run in text_buffer.layout_runs() {
            for glyph in run.glyphs.iter() {
                let physical_glyph = glyph.physical((rect.x, rect.y), 1.0);
                
                if let Some((glyph_buffer, placement)) = self.glyph_cache.allocate(
                    physical_glyph.cache_key,
                    color_rgb,
                    &mut font_system,
                    &mut swash,
                ) {
                    // Draw the rasterized glyph with proper alpha blending
                    let glyph_x = physical_glyph.x + placement.left;
                    let glyph_y = physical_glyph.y - placement.top + run.line_y as i32;
                    
                    for y in 0..placement.height as i32 {
                        for x in 0..placement.width as i32 {
                            let pixel_x = glyph_x + x;
                            let pixel_y = glyph_y + y;
                            
                            if pixel_x >= 0 && pixel_x < buf_width as i32 && 
                               pixel_y >= 0 && pixel_y < buf_height as i32 {
                                let glyph_idx = (y * placement.width as i32 + x) as usize;
                                let buffer_idx = (pixel_y as u32 * buf_width + pixel_x as u32) as usize;
                                
                                if glyph_idx < glyph_buffer.len() && buffer_idx < buffer.len() {
                                    let glyph_pixel = glyph_buffer[glyph_idx];
                                    if glyph_pixel >> 24 > 0 { // Check alpha channel
                                        // Simple alpha blending
                                        let alpha = (glyph_pixel >> 24) as f32 / 255.0;
                                        let inv_alpha = 1.0 - alpha;
                                        
                                        let bg = buffer[buffer_idx];
                                        let bg_r = ((bg >> 16) & 0xFF) as f32;
                                        let bg_g = ((bg >> 8) & 0xFF) as f32;
                                        let bg_b = (bg & 0xFF) as f32;
                                        
                                        let fg_r = ((glyph_pixel >> 16) & 0xFF) as f32;
                                        let fg_g = ((glyph_pixel >> 8) & 0xFF) as f32;
                                        let fg_b = (glyph_pixel & 0xFF) as f32;
                                        
                                        let r = (fg_r * alpha + bg_r * inv_alpha) as u32;
                                        let g = (fg_g * alpha + bg_g * inv_alpha) as u32;
                                        let b = (fg_b * alpha + bg_b * inv_alpha) as u32;
                                        
                                        buffer[buffer_idx] = (r << 16) | (g << 8) | b;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        self.glyph_cache.trim();
    }

    fn draw_background(
        &self,
        background: &Background,
        rect: Rectangle,
        buffer: &mut [u32],
        buf_width: u32,
        buf_height: u32,
    ) {
        match background {
            Background::Color(color) => {
                let color_u32 = ((color.a * 255.0) as u32) << 24
                    | ((color.r * 255.0) as u32) << 16
                    | ((color.g * 255.0) as u32) << 8
                    | ((color.b * 255.0) as u32);

                let x_start = rect.x.max(0.0) as u32;
                let y_start = rect.y.max(0.0) as u32;
                let x_end = (rect.x + rect.width).min(buf_width as f32) as u32;
                let y_end = (rect.y + rect.height).min(buf_height as f32) as u32;

                for y in y_start..y_end {
                    for x in x_start..x_end {
                        let idx = (y * buf_width + x) as usize;
                        if idx < buffer.len() {
                            buffer[idx] = color_u32;
                        }
                    }
                }
            }
        }
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

impl TextMeasurer for Renderer {
    fn measure(&self, text: &str, font_size: f32) -> Size {
        self.measure_text(text, font_size)
    }
}

impl RendererTrait for Renderer {
    fn start_layer(&mut self, _bounds: Rectangle) {
        // For now, layers are not implemented
        // This would require maintaining a stack of clip regions
    }

    fn end_layer(&mut self) {
        // For now, layers are not implemented
    }

    fn start_transformation(&mut self, _transformation: Transformation) {
        // For now, transformations are not implemented
        // This would require maintaining a transformation stack
    }

    fn end_transformation(&mut self) {
        // For now, transformations are not implemented
    }

    fn fill_quad(&mut self, quad: Quad, background: impl Into<Background>) {
        // For now, quads are not implemented
        // This would require drawing rectangles with borders and shadows
        let _ = (quad, background);
    }

    fn reset(&mut self, _new_bounds: Rectangle) {
        // Reset is handled by the draw() method which clears the buffer
    }

    fn hint(&mut self, _scale_factor: f32) {
        // Scale factor hinting is not implemented yet
    }

    fn scale_factor(&self) -> Option<f32> {
        None
    }

    fn tick(&mut self) {
        // No async operations to poll
    }
}

// Extension methods for drawing with Layout
impl Renderer {
    /// Draw text at the given layout position.
    /// This is a helper method for widgets that use the Layout system.
    pub fn draw_text_at_layout(
        &mut self,
        text: &str,
        font_size: f32,
        layout: crate::core::Layout<'_>,
        buffer: &mut [u32],
        buf_width: u32,
        buf_height: u32,
    ) {
        let bounds = layout.bounds();
        self.draw_text(text, bounds, font_size, Color::new(0.0, 0.0, 0.0, 1.0), buffer, buf_width, buf_height);
    }
}

#[derive(Debug, Clone, Default)]
struct GlyphCache {
    entries: HashMap<(cosmic_text::CacheKey, [u8; 3]), (Vec<u32>, cosmic_text::Placement)>,
    recently_used: HashSet<(cosmic_text::CacheKey, [u8; 3])>,
    trim_count: usize,
}

impl GlyphCache {
    const TRIM_INTERVAL: usize = 300;
    const CAPACITY_LIMIT: usize = 16 * 1024;

    fn new() -> Self {
        GlyphCache::default()
    }

    fn allocate(
        &mut self,
        cache_key: cosmic_text::CacheKey,
        color: [u8; 3],
        font_system: &mut cosmic_text::FontSystem,
        swash: &mut cosmic_text::SwashCache,
    ) -> Option<(&[u32], cosmic_text::Placement)> {
        let key = (cache_key, color);
        self.recently_used.insert(key);

        // Check if already cached
        if self.entries.contains_key(&key) {
            let (buffer, placement) = self.entries.get(&key).unwrap();
            return Some((buffer, *placement));
        }

        // Not cached, need to rasterize
        let image = swash.get_image_uncached(font_system, cache_key)?;
        let glyph_size = image.placement.width as usize * image.placement.height as usize;

        if glyph_size == 0 {
            return None;
        }

        let mut buffer = vec![0u32; glyph_size];
        let [r, g, b] = color;

        match image.content {
            cosmic_text::SwashContent::Mask => {
                let mut i = 0;
                for _y in 0..image.placement.height {
                    for _x in 0..image.placement.width {
                        let alpha = image.data[i];
                        if alpha > 0 {
                            buffer[i] = ((alpha as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                        }
                        i += 1;
                    }
                }
            }
            cosmic_text::SwashContent::Color => {
                let mut i = 0;
                for _y in 0..image.placement.height {
                    for _x in 0..image.placement.width {
                        buffer[i >> 2] = ((image.data[i + 3] as u32) << 24) |
                                       ((image.data[i + 2] as u32) << 16) |
                                       ((image.data[i + 1] as u32) << 8) |
                                        (image.data[i] as u32);
                        i += 4;
                    }
                }
            }
            cosmic_text::SwashContent::SubpixelMask => {
                let mut i = 0;
                for _y in 0..image.placement.height {
                    for _x in 0..image.placement.width {
                        let alpha = image.data[i];
                        if alpha > 0 {
                            buffer[i] = ((alpha as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                        }
                        i += 1;
                    }
                }
            }
        }

        let placement = image.placement;
        
        // Insert and return reference
        self.entries.insert(key, (buffer, placement));
        let (buffer, placement) = self.entries.get(&key).unwrap();
        Some((buffer, *placement))
    }

    fn trim(&mut self) {
        self.trim_count += 1;
        if self.trim_count % Self::TRIM_INTERVAL != 0 {
            return;
        }

        let mut to_remove = Vec::new();
        for key in self.entries.keys() {
            if !self.recently_used.contains(key) {
                to_remove.push(*key);
            }
        }

        for key in to_remove {
            self.entries.remove(&key);
        }

        self.recently_used.clear();

        if self.entries.len() > Self::CAPACITY_LIMIT {
            let excess = self.entries.len() - Self::CAPACITY_LIMIT;
            let keys: Vec<_> = self.entries.keys().take(excess).cloned().collect();
            for key in keys {
                self.entries.remove(&key);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cosmic_text_integration() {
        let mut renderer = Renderer::new();
        let _ = renderer.load_default_font();
        
        // Test text measurement
        let size = renderer.measure_text("Hello World", 16.0);
        assert!(size.width > 0.0);
        assert!(size.height > 0.0);
        println!("Measured text size: {}x{}", size.width, size.height);
        
        // Test that the font system can be accessed
        let font_system = font_system();
        let _fs = font_system.read().unwrap();
        println!("Font system initialized successfully");
    }
    
    #[test]
    fn test_text_buffer_creation() {
        let mut font_system = font_system().write().unwrap();
        let metrics = Metrics::new(16.0, 19.2);
        let mut buffer = Buffer::new(&mut font_system, metrics);
        buffer.set_size(&mut font_system, Some(200.0), Some(100.0));
        buffer.set_text(&mut font_system, "Test", &Attrs::new(), Shaping::Advanced, None);
        
        let runs: Vec<_> = buffer.layout_runs().collect();
        assert!(!runs.is_empty());
        println!("Text buffer created with {} layout runs", runs.len());
    }
}