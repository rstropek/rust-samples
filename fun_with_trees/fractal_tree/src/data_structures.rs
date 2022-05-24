use std::ops::{Add, AddAssign};
use serde::Serialize;

/// Implements a vector in 2D space.
/// 
/// The struct implements `serde::Serialize` if the `svg` feature is enabled.
/// This is necessary because the *handlebars* crate is used to generate
/// the SVG output and the trait is required for it.
#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "svg", derive(Serialize))]
pub struct Vector2d {
    pub x: f64,
    pub y: f64,
}

impl Add for Vector2d {
    type Output = Vector2d;
    
    /// Adds two vectors.
    fn add(self, rhs: Self) -> Self::Output {
        Vector2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// Implements a line in 2D space from a starting point to an endpoint.
/// 
/// The struct implements `serde::Serialize` if the `svg` feature is enabled.
/// This is necessary because the *handlebars* crate is used to generate
/// the SVG output and the trait is required for it.
#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(feature = "svg", derive(Serialize))]
pub struct Line {
    pub start: Vector2d,
    pub end: Vector2d,
}

/// Implements a size in 2D space.
/// 
/// The struct implements `serde::Serialize` if the `svg` feature is enabled.
/// This is necessary because the *handlebars* crate is used to generate
/// the SVG output and the trait is required for it.
#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(feature = "svg", derive(Serialize))]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

/// Implements a rectangle consisting of left upper corner, right lower corner, width, and height.
/// 
/// The struct implements `serde::Serialize` if the `svg` feature is enabled.
/// This is necessary because the *handlebars* crate is used to generate
/// the SVG output and the trait is required for it.
#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(feature = "svg", derive(Serialize))]
pub struct Rect {
    pub left_upper: Vector2d,
    pub right_lower: Vector2d,
    pub size: Size,
}

impl AddAssign<&Line> for Rect {
    /// Enlarges rectangle so that it contains the given line.
    fn add_assign(&mut self, l: &Line) {
        if l.start.x < self.left_upper.x { self.left_upper.x = l.start.x };
        if l.end.x < self.left_upper.x { self.left_upper.x = l.end.x };
        if l.start.x > self.right_lower.x { self.right_lower.x = l.start.x };
        if l.end.x > self.right_lower.x { self.right_lower.x = l.end.x };
        if l.start.y < self.left_upper.y { self.left_upper.y = l.start.y };
        if l.end.y < self.left_upper.y { self.left_upper.y = l.end.y };
        if l.start.y > self.right_lower.y { self.right_lower.y = l.start.y };
        if l.end.y > self.right_lower.y { self.right_lower.y = l.end.y };
        self.size.width = self.right_lower.x - self.left_upper.x;
        self.size.height = self.right_lower.y - self.left_upper.y;
    }
}

/// Implements a fractal tree.
/// 
/// For every level, `lines` contains one entry. The branches of the level
/// in the nested vector.
pub struct Tree {
    pub lines: Vec<Vec<Line>>,
    pub bounding_rect: Rect,
}