mod utils;

use std::ops::AddAssign;

use js_sys::Math::*;
use wasm_bindgen::prelude::*;

#[cfg(feature = "console_log")]
use web_sys::console;

use serde::{Serialize, Deserialize};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Try 1 to see exception from Result
static MAX_ROCKETS: usize = 100;
static PARTICLES_PER_ROCKET: usize = 300;
static PARTICLE_LIFESPAN: f64 = 150f64;

// Represents a rocket. Note use of serde for JSON support
#[derive(Serialize, Deserialize)]
pub struct Rocket {
    pub rgb: u32,
    pub x: f64,
    pub height: f64,
}

// Note `wasm_bindgen` macro here. Generates the necessary code
// that makes type available in JS.
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RocketState {
    Inactive = 0,
    Rising = 1,
    Exploded = 2,
}

// Note the implementation of a "Trait". In this case, we implement
// the `Default` trait (https://doc.rust-lang.org/std/default/trait.Default.html).
impl Default for RocketState {
    fn default() -> Self {
        RocketState::Inactive
    }
}

// In our application, we will use a vector of states, so we define
// a type for that.
type States = Vec<RocketState>;

// When we have our own type, we can add our own traits. Here,
// we are going to add a method that finds the index of the first
// inactive rocket in our buffer of rockets.
trait StatesExt {
    fn first_inactive(&self) -> Result<usize, JsValue>;
}

impl StatesExt for States {
    /// Returns the index of the first inactive rocket
    ///
    /// Returns the index of the first inactive rocket. If there is
    /// no inactive rocket, the method returns an error.
    fn first_inactive(&self) -> Result<usize, JsValue> {
        // Note the use of an iterator in the next line
        let free_index = self.iter().position(|&s| s == RocketState::Inactive);

        // Note match syntax over an Option enum.
        match free_index {
            None => return Err("No inactive element".into()),
            Some(ix) => Result::Ok(ix),
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParticleState {
    Inactive = 0,
    Active = 1,
}

impl Default for ParticleState {
    fn default() -> Self {
        ParticleState::Inactive
    }
}

#[wasm_bindgen]
#[derive(Clone, Default, Copy)]
pub struct Vector2d {
    x: f64,
    y: f64,
}

// Implement `AddAssign` trait in order to get `+=` operator 
// for `Vector2d` (https://doc.rust-lang.org/std/ops/trait.AddAssign.html)
impl AddAssign for Vector2d {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[wasm_bindgen]
#[derive(Default, Clone)]
pub struct RocketColor {
    red: u8,
    green: u8,
    blue: u8,
}

#[wasm_bindgen]
#[derive(Default, Clone)]
pub struct RocketParameter {
    spread: f64,
    velocity: f64,
    acceleration: f64,
}

#[wasm_bindgen]
#[derive(Default, Clone)]
pub struct ParticleParameter {
    velocity: Vector2d,
    lifespan: f64,
    lifespan_diff: f64,
    acceleration: f64,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct Firework {
    rocket_states: States,
    colors: Vec<RocketColor>,
    rockets: Vec<Vector2d>,
    parameters: Vec<RocketParameter>,
    particle_states: Vec<ParticleState>,
    particles: Vec<Vector2d>,
    particle_parameters: Vec<ParticleParameter>,
}

impl Firework {
    /// Adds a particle for a rocket
    ///
    /// # Arguments
    ///
    /// * `rix` - zero-based index of rocket
    /// * `pix` - zero-based particle index
    pub fn add_particle(&mut self, rix: usize, pix: usize) {
        // Every particle starts at position where rocket exploded
        let r = self.rockets[rix];

        // Note the use of a local function
        fn rand_velocity(spread: f64) -> f64 {
            (random() * 2f64 - 1f64) * random() * spread
        }

        let spread = self.parameters[rix].spread;
        let p = ParticleParameter {
            velocity: Vector2d {
                x: rand_velocity(spread),
                y: rand_velocity(spread),
            },
            lifespan: PARTICLE_LIFESPAN,
            lifespan_diff: random() * 6.5f64 + 1.5f64,
            acceleration: 0f64,
        };

        // Calculate particle index
        // Note "shadowing" of local variable pix here.
        let pix = rix * PARTICLES_PER_ROCKET + pix;

        // Set particle state, position, and parameters
        self.particle_states[pix] = ParticleState::Active;
        self.particles[pix] = r;
        self.particle_parameters[pix] = p;
    }

    /// Let rocket explode
    ///
    /// # Arguments
    ///
    /// * `rix` - zero-based index of rocket
    pub fn explode(&mut self, rix: usize) {
        // Generate particles for exploding rocket
        for pix in 0..PARTICLES_PER_ROCKET {
            self.add_particle(rix, pix);
        }

        // Note how we conditionally call JavaScript's console.log
        #[cfg(feature = "console_log")]
        console::log_3(&"Rocket ".into(), &rix.to_string().into(), &" exploded".into());

        // Change rocket state to exploded
        self.rocket_states[rix] = RocketState::Exploded;
    }
}

// Here we have functions that should be exported to JavaScript.
#[wasm_bindgen]
impl Firework {
    /// Create a new firework
    pub fn new() -> Firework {
        Firework {
            rocket_states: vec![Default::default(); MAX_ROCKETS],
            colors: vec![Default::default(); MAX_ROCKETS],
            rockets: vec![Default::default(); MAX_ROCKETS],
            parameters: vec![Default::default(); MAX_ROCKETS],
            particle_states: vec![Default::default(); MAX_ROCKETS * PARTICLES_PER_ROCKET],
            particles: vec![Default::default(); MAX_ROCKETS * PARTICLES_PER_ROCKET],
            particle_parameters: vec![Default::default(); MAX_ROCKETS * PARTICLES_PER_ROCKET],
        }
    }

    /// Add a rocket
    ///
    /// # Arguments
    /// 
    /// * `red` - Red portion of RGB color (0..255)
    /// * `green` - Green portion of RGB color (0..255)
    /// * `blue` - Green portion of RGB color (0..255)
    /// * `x` - x coordinate where rocket should rise
    /// * `height` - Approximate height to which the rocket should rise before exploding
    ///
    /// # Remarks
    ///
    /// Throws an exception if all available rocket slots are currently in use.
    pub fn add(&mut self, red: u8, green: u8, blue: u8, x: f64, height: f64) -> Result<(), JsValue> {
        // Note the `?` at the end of the line. Will return an error
        // if `first_inactive` returned an error.
        let free_index = self.rocket_states.first_inactive()?;

        // Calculate rocket parameters
        let p = RocketParameter {
            spread: random() * 20f64 + 10f64,
            velocity: (height.sqrt() * -1f64) / 2f64 * f64::from(height) / 100f64,
            acceleration: 0f64,
        };
        let c = RocketColor { red, green, blue };
        let r = Vector2d { x, y: 0f64 };

        // Set rocket parameters
        self.rocket_states[free_index] = RocketState::Rising;
        self.colors[free_index] = c;
        self.rockets[free_index] = r;
        self.parameters[free_index] = p;

        // Note how we conditionally call JavaScript's console.log
        #[cfg(feature = "console_log")]
        console::log_2(&"Added 🚀 at index ".into(), &free_index.to_string().into());

        Result::Ok(())
    }

    /// Calculate new rocket state
    pub fn apply_force(&mut self) {
        // Iterate over all rockets
        for rix in 0..self.rockets.len() {
            // Ignore inactive rockets
            let rocket_state = self.rocket_states[rix];
            if rocket_state == RocketState::Inactive {
                continue;
            }

            // Borrow mutable parameter of current rocket and calculate
            // new values for y coordinate and parameters.
            let parameter = &mut self.parameters[rix];
            self.rockets[rix].y += parameter.velocity;
            parameter.velocity += parameter.acceleration;
            parameter.acceleration += 0.005f64;

            // Check if rocket has started to fall down again -> let it explode
            if parameter.velocity > 0f64 && rocket_state == RocketState::Rising {
                self.explode(rix);
            }

            // If rocket has exploded, deal with particles
            if self.rocket_states[rix] == RocketState::Exploded {
                let mut at_least_one_particle_active = false;
                for pix in 0..PARTICLES_PER_ROCKET {
                    // Calculate global index of particle. Note shadowing again.
                    let pix = rix * PARTICLES_PER_ROCKET + pix;

                    // Ignore inactive particles
                    if self.particle_states[pix] == ParticleState::Inactive {
                        continue;
                    }

                    // Note use of operator overloading
                    self.particles[pix] += self.particle_parameters[pix].velocity;

                    // Borrow mutable parameter of current particle and calculate new values.
                    let parameter = &mut self.particle_parameters[pix];
                    parameter.velocity.y += parameter.acceleration;
                    parameter.acceleration += 0.005f64;
                    parameter.velocity.x *= 0.9f64;
                    parameter.velocity.y *= 0.9f64;
                    parameter.lifespan -= parameter.lifespan_diff;

                    if parameter.lifespan <= 0f64 {
                        self.particle_states[pix] = ParticleState::Inactive;
                    } else {
                        // Found at least one particle which is still active
                        at_least_one_particle_active = true;
                    }
                }

                if !at_least_one_particle_active {
                    // No more active particles -> rocket becomes inactive
                    self.rocket_states[rix] = RocketState::Inactive;
                }
            }
        }
    }

    // Note the use of JsValue. Read more at 
    // https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html
    pub fn add_rocket(&mut self, rocket: &JsValue) -> Result<(), JsValue> {
        // Note deserialization of JSON using Serde
        let rocket: Rocket = rocket.into_serde().unwrap();

        self.add(((rocket.rgb & 0xff0000u32) >> 8 >> 8) as u8, ((rocket.rgb & 0xff00u32) >> 8) as u8, (rocket.rgb & 0xffu32) as u8, rocket.x, rocket.height)?;

        Result::Ok(())
    }

    /// Get size of rocket buffer
    #[wasm_bindgen(getter)]
    pub fn rocket_buffer_size(&self) -> usize {
        MAX_ROCKETS
    }

    /// Get size of rocket buffer
    #[wasm_bindgen(getter)]
    pub fn particle_buffer_size(&self) -> usize {
        MAX_ROCKETS * PARTICLES_PER_ROCKET
    }

    // Calculate index of praticle based on rocket and particle ID
    pub fn get_particle_index(rix: usize, pix: usize) -> usize {
        rix * PARTICLES_PER_ROCKET + pix
    }
    
    /// Get buffer of rocket coordinates
    pub fn rockets(&self) -> *const Vector2d {
        self.rockets.as_ptr()
    }

    /// Get buffer of rocket colors
    pub fn colors(&self) -> *const RocketColor {
        self.colors.as_ptr()
    }

    /// Get buffer of rocket states
    pub fn states(&self) -> *const RocketState {
        self.rocket_states.as_ptr()
    }

    /// Get buffer of particle states
    pub fn particle_states(&self) -> *const ParticleState {
        self.particle_states.as_ptr()
    }

    /// Get buffer of particle coordinates
    pub fn particles(&self) -> *const Vector2d {
        self.particles.as_ptr()
    }

    /// Get max particles per rocket
    pub fn get_max_particles_per_rocket(&self) -> usize {
        PARTICLES_PER_ROCKET
    }
}
