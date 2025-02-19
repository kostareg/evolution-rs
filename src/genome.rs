use crate::neurons::*;

use rand::Rng;

pub type Genomes = [Genome; 8]; // for a base: 8 genomes.

#[derive(Debug, Copy, Clone)]
pub struct Genome {
    pub source: Source,
    pub sink: Sink,
    pub weight: i8, // from -10 to 10 for now.
}

impl Genome {
    pub fn random_new() -> Self {
        let mut rng = rand::rng();

        Self {
            source: Source::random_new(),
            sink: Sink::random_new(),
            weight: rng.random_range(-10..=10),
        }
    }
}
