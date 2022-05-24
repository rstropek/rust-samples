use rand::Rng;
use rand_core::RngCore;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

mod data_structures;
#[cfg(feature = "svg")]
mod svg;

pub use data_structures::*;
#[cfg(feature = "svg")]
pub use svg::*;

/// Generates a branch (=line) for the fractal tree.
/// 
/// # Arguments:
/// 
/// * `lines` - The lines representing the branches of the tree.
/// * `start` - Starting point of the branch.
/// * `angle` - Angle of the line representing the branch.
/// * `branch_length` - Length of the line representing the branch.
/// * `level` - Level of the branch (0 = trunk, 1 = first branch, etc.).
/// * `rng` - Random number generator.
fn branch(
    lines: &mut [Vec<Line>],
    start: Vector2d,
    angle: f64,
    mut branch_length: f64,
    mut level: usize,
    rng: &mut impl RngCore,
) {
    // Some parameter influencing the visual appearance of the tree
    const BRANCH_SHORTENING_FACTOR: f64 = 0.8f64;
    const RANDOM_ANGLE_CHANGE_FACTOR: f64 = 35f64;
    const RANDOM_BRANCH_LENGTH_CHANGE_FACTOR: f64 = 25f64;
    const ANGLE_CHANGE: f64 = 20f64 * std::f64::consts::PI / 180f64;

    debug_assert!(level < lines.len(), "level is out of boundes, must be lower than {}", lines.len());

    // Calculate the end point of the branch
    let end = start
        + Vector2d {
            x: angle.cos() * branch_length,
            y: angle.sin() * branch_length,
        };

    // Add the line representing the branch to the lines
    lines[level].push(Line { start, end });

    // If we did not yet reach the maximum level, create a new branch
    level += 1;
    if level < lines.len() {
        // Shorten the branch
        branch_length *= BRANCH_SHORTENING_FACTOR;

        // Add a little bit of randomness to the branch's angle and length
        let angle_change =
            (rng.gen_range(-0.5f64..0.5f64) * RANDOM_ANGLE_CHANGE_FACTOR).to_radians();
        let branch_length_change =
            rng.gen_range(-0.5f64..0.5f64) * RANDOM_BRANCH_LENGTH_CHANGE_FACTOR;

        // Generate two new branches recursively
        branch(
            lines,
            end,
            angle - ANGLE_CHANGE + angle_change,
            branch_length + branch_length_change,
            level,
            rng,
        );
        branch(
            lines,
            end,
            angle + ANGLE_CHANGE + angle_change,
            branch_length + branch_length_change,
            level,
            rng,
        );
    }
}

/// Generates a fractal tree.
/// 
/// # Arguments:
/// 
/// * `seed` - Seed for the random number generator.
/// * `levels` - Number of levels of the tree.
/// * `trunk_length` - Length of the trunk of the tree.
pub fn tree<S: AsRef<str>>(seed: S, levels: usize, trunk_length: f64) -> Tree {
    // Generate a seeded random number generator
    let mut rng: Pcg64 = Seeder::from(seed.as_ref()).make_rng();

    // Allocate the tree data structure. It fills `lines` with 
    // the necessary number of vectors (one for each level of the tree).
    let mut tree = Tree {
        lines: (0..levels).into_iter().map(|_| Vec::new()).collect(),
        bounding_rect: Default::default(),
    };

    // Generate the trunk of the tree
    branch(
        &mut tree.lines,
        Default::default(),
        -90f64.to_radians(),
        trunk_length,
        0,
        &mut rng,
    );

    // Calculate the bounding rectangle of the tree
    let mut bounding_rect: Rect = Default::default();
    tree.lines.iter().for_each(|lines| {
        lines.iter().for_each(|line| {
            bounding_rect += line;
        });
    });
    tree.bounding_rect = bounding_rect;

    tree
}
