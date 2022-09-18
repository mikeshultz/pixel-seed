pub mod drawing;
pub mod pattern;
pub mod utils;

use draw::RGB;
use std::fmt;
use std::io;

type Grid = Vec<Vec<Pixel>>;

const CANVAS_SIZE: u32 = 500;
const PIXEL_SIZE: u32 = 25;
const PATTERN_SIZE_BYTES: u32 = 4096;

/// An RGBA color (TODO: A is currently unused)
#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Create a Color from a hex string
    pub fn from_hex(s: &str) -> Self {
        let s = utils::remove_0x(s);
        let [r, g, b, a] = utils::decode_hex_color(s);
        Self { r, g, b, a }
    }

    /// Create an rgb::RBG instance for this Color
    pub fn to_rgb(&self) -> RGB {
        RGB {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }

    /// Create a black Color instance
    pub fn black() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

/// Details about a single pixel
#[derive(Clone, Debug)]
pub struct Pixel {
    pub color: Color,
}

/// Configuration informing how we will draw
#[derive(Debug)]
pub struct Config {
    pub canvas_size: u32,
    pub pixel_size: u32,
    pub start_pos: (u32, u32),
    pub color_bg: Color,
    pub color_fg: Color,
    pub grid: Grid,
}

impl<'a> Config {
    pub fn from_seed(s: &'a str) -> Self {
        let seed = utils::remove_0x(s);
        let pixel_count = CANVAS_SIZE / PIXEL_SIZE;
        let uno = hex::decode(&s[6..8]).unwrap()[0] as u32;
        let dos = hex::decode(&s[9..11]).unwrap()[0] as u32;
        let start_pos = (uno % pixel_count, dos % pixel_count);
        let color_bg = Color::from_hex(&seed[0..8]);

        Self {
            canvas_size: CANVAS_SIZE,
            pixel_size: PIXEL_SIZE,
            start_pos,
            color_bg,
            color_fg: Color::from_hex(&seed[56..64]),
            grid: vec![vec![Pixel { color: color_bg }; pixel_count as usize]; pixel_count as usize],
        }
    }
}

/// Parse a given seed and generate a Config
pub fn parse_seed(seed: &str) -> Result<Config, io::Error> {
    let mut config = Config::from_seed(seed);
    mutate_grid(seed, &mut config);
    Ok(config)
}

/// Given a clean drawing grid, mutate it according to the seed and our
/// arbitrary rules.
pub fn mutate_grid<'a, 'b>(seed: &'a str, config: &'b mut Config) {
    let mut cursor = drawing::Cursor::from_config(config);
    let normal = utils::remove_0x(seed);
    let draw_pattern = pattern::seed_to_pattern(&normal[8..56], PATTERN_SIZE_BYTES);

    // TODO: Should set initial direction from the seed
    let mut dir = drawing::Direction::South;

    for chunk in utils::chunk_hex_str(&draw_pattern) {
        // Get the current magic number for drawing derivation
        let num = hex::decode(&chunk).unwrap()[0];
        // Using the last bit of num to decide if the current pixel is active
        let active = (num & 0b01u8) == 1;

        // Amount of of "divisible by" in a range of 0..255:
        //
        // ones: 85   - Forward
        // twos: 43   - U-turn
        // threes: 63 - Left
        // fours: 64  - Right
        //
        // Figure out the direction we've moving
        dir = match num {
            i if i % 4 == 0 => dir.turn(drawing::Turn::Right),
            i if i % 3 == 0 => dir.turn(drawing::Turn::Left),
            i if i % 2 == 0 => dir.turn(drawing::Turn::Around),
            _ => dir,
        };

        config.grid[cursor.x][cursor.y].color = if active {
            config.color_fg
        } else {
            config.color_bg
        };

        cursor.travel(dir);
    }
}
