use crate::genome::*;

use rand::Rng;

/// A blob is a 1x1 creature.
///
/// The x and y positions are measured as a percentage from the center of the
/// board, in order to be easier to fit in a range of -1 to 1.
/// `(x, y) = (-1, -1)` is the bottom left of the board.
#[derive(Debug, Copy, Clone)]
pub struct Blob {
    pub x: f32,
    pub y: f32,
    pub genomes: Genomes,
    pub internal_state: InternalState,
}

#[derive(Debug, Copy, Clone)]
pub struct InternalState {
    pub I0: f32,
    pub I1: f32,
    pub I2: f32,
    pub I3: f32,
}

impl Blob {
    /// Create a randomly generated new blob. Only used in 0th generation.
    pub fn random_new() -> Self {
        let mut rng = rand::rng();

        Self {
            x: rng.random_range(-1. ..= 1.),
            y: rng.random_range(-1. ..= 1.),
            genomes: std::array::from_fn(|_| Genome::random_new()),
            internal_state: InternalState {
                I0: rng.random_range(-1. ..= 1.),
                I1: rng.random_range(-1. ..= 1.),
                I2: rng.random_range(-1. ..= 1.),
                I3: rng.random_range(-1. ..= 1.),
            }
        }
    }

    pub fn random_pos(genomes: Genomes) -> Self {
        let mut rng = rand::rng();

        Self {
            x: rng.random_range(-1. ..= 1.),
            y: rng.random_range(-1. ..= 1.),
            genomes,
            internal_state: InternalState {
                I0: rng.random_range(-1. ..= 1.),
                I1: rng.random_range(-1. ..= 1.),
                I2: rng.random_range(-1. ..= 1.),
                I3: rng.random_range(-1. ..= 1.),
            }
        }
    }
}
