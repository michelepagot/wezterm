// Clippy hates the implement_vertex macro and won't let me scope
// this warning to its use
#![allow(clippy::unneeded_field_pattern)]

use ::window::bitmaps::TextureRect;
use ::window::color::LinearRgba;

/// Each cell is composed of two triangles built from 4 vertices.
/// The buffer is organized row by row.
pub const VERTICES_PER_CELL: usize = 4;
pub const V_TOP_LEFT: usize = 0;
pub const V_TOP_RIGHT: usize = 1;
pub const V_BOT_LEFT: usize = 2;
pub const V_BOT_RIGHT: usize = 3;

#[derive(Copy, Clone, Default)]
pub struct Vertex {
    // Physical position of the corner of the character cell
    pub position: (f32, f32),
    // glyph texture
    pub tex: (f32, f32),
    pub fg_color: (f32, f32, f32, f32),
    pub alt_color: (f32, f32, f32, f32),
    pub hsv: (f32, f32, f32),
    // We use a float for this because I can't get
    // bool or integer values to work:
    // "bool can't be an in in the vertex shader"
    //
    // has_color is effectively an enum with these
    // possible values:
    // 0.0 -> a regular monochrome text glyph
    // 1.0 -> a color emoji glyph
    // 2.0 -> a full color texture attached as the
    //        background image of the window
    // 3.0 -> like 2.0, except that instead of an
    //        image, we use the solid bg color
    pub has_color: f32,
    pub mix_value: f32,
}
::window::glium::implement_vertex!(
    Vertex, position, tex, fg_color, alt_color, hsv, has_color, mix_value
);

/// A helper for updating the 4 vertices that compose a glyph cell
pub struct Quad<'a> {
    pub(crate) vert: &'a mut [Vertex],
}

impl<'a> Quad<'a> {
    /// Assign the texture coordinates
    pub fn set_texture(&mut self, coords: TextureRect) {
        let x1 = coords.min_x();
        let x2 = coords.max_x();
        let y1 = coords.min_y();
        let y2 = coords.max_y();
        self.set_texture_discrete(x1, x2, y1, y2);
    }

    pub fn set_texture_discrete(&mut self, x1: f32, x2: f32, y1: f32, y2: f32) {
        self.vert[V_TOP_LEFT].tex = (x1, y1);
        self.vert[V_TOP_RIGHT].tex = (x2, y1);
        self.vert[V_BOT_LEFT].tex = (x1, y2);
        self.vert[V_BOT_RIGHT].tex = (x2, y2);
    }

    /// Set the color glyph "flag"
    pub fn set_has_color(&mut self, has_color: bool) {
        let has_color = if has_color { 1. } else { 0. };
        for v in self.vert.iter_mut() {
            v.has_color = has_color;
        }
    }

    /// Mark as a grayscale polyquad; color and alpha will be
    /// multipled with those in the texture
    pub fn set_grayscale(&mut self) {
        for v in self.vert.iter_mut() {
            v.has_color = 4.0;
        }
    }

    /// Mark this quad as a background image.
    /// Mutually exclusive with set_has_color.
    pub fn set_is_background_image(&mut self) {
        for v in self.vert.iter_mut() {
            v.has_color = 2.0;
        }
    }

    pub fn set_is_background(&mut self) {
        for v in self.vert.iter_mut() {
            v.has_color = 3.0;
        }
    }

    pub fn set_fg_color(&mut self, color: LinearRgba) {
        for v in self.vert.iter_mut() {
            v.fg_color = color.tuple();
        }
        self.set_alt_color_and_mix_value(color, 0.);
    }

    /// Must be called after set_fg_color
    pub fn set_alt_color_and_mix_value(&mut self, color: LinearRgba, mix_value: f32) {
        for v in self.vert.iter_mut() {
            v.alt_color = color.tuple();
            v.mix_value = mix_value;
        }
    }

    pub fn set_hsv(&mut self, hsv: Option<config::HsbTransform>) {
        let s = hsv
            .map(|t| (t.hue, t.saturation, t.brightness))
            .unwrap_or((1., 1., 1.));
        for v in self.vert.iter_mut() {
            v.hsv = s;
        }
    }

    #[allow(unused)]
    pub fn get_position(&self) -> (f32, f32, f32, f32) {
        let top_left = self.vert[V_TOP_LEFT].position;
        let bottom_right = self.vert[V_BOT_RIGHT].position;
        (top_left.0, top_left.1, bottom_right.0, bottom_right.1)
    }

    pub fn set_position(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        self.vert[V_TOP_LEFT].position = (left, top);
        self.vert[V_TOP_RIGHT].position = (right, top);
        self.vert[V_BOT_LEFT].position = (left, bottom);
        self.vert[V_BOT_RIGHT].position = (right, bottom);
    }
}
