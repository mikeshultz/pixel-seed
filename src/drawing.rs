use draw::{render, Canvas, Drawing, Shape, Style, SvgRenderer};
use std::io;

use super::Config;

/// Cardinal direction of travel
#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Return a Direction as if making a relative directional turn
    pub fn turn(&self, relative: Turn) -> Self {
        match self {
            Self::North => match relative {
                Turn::Around => Self::South,
                Turn::Left => Self::West,
                _ => Self::East,
            },
            Self::South => match relative {
                Turn::Around => Self::North,
                Turn::Left => Self::East,
                _ => Self::West,
            },
            Self::East => match relative {
                Turn::Around => Self::West,
                Turn::Left => Self::North,
                _ => Self::South,
            },
            Self::West => match relative {
                Turn::Around => Self::East,
                Turn::Left => Self::South,
                _ => Self::North,
            },
        }
    }
}

/// A relative direction change
#[derive(Copy, Clone, Debug)]
pub enum Turn {
    Left,
    Right,
    Around,
}

/// A representatio of a cursor on a grid of pixels
pub struct Cursor {
    boundary: usize,
    pub x: usize,
    pub y: usize,
}

impl<'a> Cursor {
    /// Create a cursor (current place on a drawing grid)
    pub fn from_config(config: &'a Config) -> Self {
        let boundary = (config.canvas_size / config.pixel_size) as usize;

        let mut new_cursor = Self {
            boundary,
            x: config.start_pos.0 as usize,
            y: config.start_pos.1 as usize,
        };

        new_cursor._boundary_correct();

        new_cursor
    }

    /// Move the cursor in a cardinal direction
    pub fn travel(&mut self, dir: Direction) {
        let (new_x, new_y): (usize, usize) = match dir {
            Direction::North => (self.x + 1, self.y),
            Direction::South => (if self.x > 0 { self.x - 1 } else { 0 }, self.y),
            Direction::East => (self.x, self.y + 1),
            Direction::West => (self.x, if self.y > 0 { self.y - 1 } else { 0 }),
        };

        self.x = new_x;
        self.y = new_y;

        // Make sure we don't go out of bounds
        self._boundary_correct();
    }

    /// Set the max-pixel boundary for a square grid
    pub fn set_boundary(&mut self, bound: usize) {
        self.boundary = bound;

        // Make sure we don't go out of bounds
        self._boundary_correct();
    }

    /// Return the coordinates of the cursor's position
    pub fn to_tuple(&self) -> (&usize, &usize) {
        (&self.x, &self.y)
    }

    /// Correct the current cursor position to within the allowed boundaries
    fn _boundary_correct(&mut self) {
        if self.boundary > 0 {
            if self.x >= self.boundary {
                self.x = self.boundary - 1;
            }
            if self.y >= self.boundary {
                self.y = self.boundary - 1;
            }
        }
    }
}

/// Get the position of a false "pixel" (a drawn square) in a grid of actual
/// pixels.
fn pos(config: &Config, x: &u32, y: &u32) -> (u32, u32) {
    (x * config.pixel_size, y * config.pixel_size)
}

/// Draw an image on a canvas according to the Config
pub fn draw_from_config(config: &Config) -> Result<(), io::Error> {
    println!("draw_from_config()");
    let mut canvas = Canvas::new(config.canvas_size, config.canvas_size);

    for (x, row) in (0u32..).zip(config.grid.iter()) {
        for (y, pixel) in (0u32..).zip(row.iter()) {
            let (pos_x, pos_y) = pos(config, &x, &y);

            canvas.display_list.add(
                Drawing::new()
                    // give it a shape
                    .with_shape(Shape::Rectangle {
                        width: config.pixel_size,
                        height: config.pixel_size,
                    })
                    // move it around
                    .with_xy(pos_x as f32, pos_y as f32)
                    // give it a cool style
                    .with_style(Style::filled(pixel.color.to_rgb())),
            );
        }
    }

    // save the canvas as an svg
    render::save(&canvas, "tmp/pixel_seed_test.svg", SvgRenderer::new())
}
