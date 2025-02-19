use crate::genome::*;

use rand::Rng;

/// A blob is a 1x1 creature.
///
/// The x and y positions are measured as a percentage from the center of the
/// board, in order to be easier to fit in a range of -1 to 1.
/// `(x, y) = (-1, -1)` is the bottom left of the board.
#[derive(Debug)]
pub struct Blob {
    pub x: f32,
    pub y: f32,
    pub genomes: Genomes,
}

impl Blob {
    /// Create a randomly generated new blob. Only used in 0th generation.
    pub fn random_new() -> Self {
        let mut rng = rand::rng();

        Self {
            x: rng.random_range(-1. ..= 1.),
            y: rng.random_range(-1. ..= 1.),
            genomes: std::array::from_fn(|_| Genome::random_new()),
        }
    }
}
