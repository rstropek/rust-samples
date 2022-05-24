use serde::Serialize;
use handlebars::{handlebars_helper, Handlebars};

use crate::*;

/// Represents settings for a line in the tree image
#[derive(Debug, Serialize)]
pub struct LineSetting {
    /// Lightness percentage (HSL color mode)
    pub lightness: u8,

    /// Width of the line
    pub width: f64,
}

impl LineSetting {
    /// Creates line settings from a given tree branching level.
    /// 
    /// # Arguments
    /// 
    /// * `level` - The current level.
    /// * `max_level` - The total number of levels in the tree.
    fn new(level: usize, max_level: usize) -> Self {
        const LEVEL_TO_LIGHTNESS_FACTOR: u8 = 3u8;
        const LEVEL_TO_WEIGHT_FACTOR: f64 = 3f64;

        LineSetting {
            lightness: level as u8 * LEVEL_TO_LIGHTNESS_FACTOR,
            width: (max_level + 1 - level) as f64 * LEVEL_TO_WEIGHT_FACTOR / 2f64,
        }
    }
}

/// Represent a collection of all lines of a common tree level.
/// 
/// Because all lines are on the same tree level, they share
/// the same settings.
#[derive(Serialize)]
struct SettingsWithLines<'a> {
    settings: LineSetting,
    lines: &'a Vec<Line>,
}

/// Contains all tree levels with their branches and bounding rect.
/// 
/// Used for SVG generation
#[derive(Serialize)]
struct TreeGenerationData<'a> {
    lines: Vec<SettingsWithLines<'a>>,
    bounding_rect: Rect,
}

const SVG_TEMPLATE: &str = r#"
<svg version="1.1"
     baseProfile="full"
     xmlns="http://www.w3.org/2000/svg"
     xmlns:xlink="http://www.w3.org/1999/xlink"
     xmlns:ev="http://www.w3.org/2001/xml-events"
     width="{{format this.bounding_rect.size.width}}" height="{{format this.bounding_rect.size.height}}" 
     viewBox="{{format this.bounding_rect.left_upper.x}} {{format this.bounding_rect.left_upper.y}} {{format this.bounding_rect.size.width}} {{format this.bounding_rect.size.height}} ">
    <g>
    {{#each this.lines}}
        <path d="{{#each lines}}M{{format start.x}},{{format start.y}} L{{format end.x}}{{format end.y}}z{{/each}} " stroke="hsl(90, 100%, {{settings.lightness}}%)" stroke-width="{{format settings.width}}" />
    {{/each}}
    </g>
</svg>"#;

pub fn tree_svg<S: AsRef<str>>(seed: S, levels: usize, trunk_length: f64) -> String {
    // Create handlebar template engine
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("svg", SVG_TEMPLATE)
        .unwrap();
    handlebars_helper!(format: |v: f64| format!("{v:.4}"));
    handlebars.register_helper("format", Box::new(format));
    
    // Generate tree
    let tree_lines = tree(seed, levels, trunk_length);

    // Convert tree data to data structure required for SVG template
    let tree = TreeGenerationData {
        lines: tree_lines.lines
            .iter()
            .enumerate()
            .map(|l| SettingsWithLines {
                settings: LineSetting::new(l.0, tree_lines.lines.len() - 1),
                lines: l.1
            })
            .collect(),
        bounding_rect: tree_lines.bounding_rect,
    };

    handlebars.render("svg", &tree).unwrap()
}
