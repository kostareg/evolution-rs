use crate::blob::*;
use crate::neurons::*;

use macroquad::prelude::*;
use ::rand::{self, Rng};
use bincode::Options;

/// The simulation environment.
pub struct Simulator {
    blobs: Vec<Blob>,
}

// TODO: how to multithread this?

impl Simulator {
    pub fn new(blobs: Vec<Blob>) -> Self {
        Self { blobs }
    }

    pub async fn run(&mut self) {
        let mut rng = rand::rng();
        for i in 1..=100 {
            self.run_generation(i).await;
        
            // delete everyone on the left.
            let blobs: Vec<_> = self.blobs.clone().into_iter().filter(|blob| blob.x.is_sign_positive()).collect();
            self.blobs = vec![];

            // use the remaining blobs to repopulate.
            for _ in 0..200 {
                // pick a random blob's genomes from the survivors and copy it.
                let r = rng.random_range(0..blobs.len());
                // println!("{}", r);
                let selected = blobs[r];
                //println!("{:#?}", selected);

                let blob = Blob::random_pos(selected.genomes);

                let config = bincode::DefaultOptions::new()
                    .with_varint_encoding()  // Uses fixed-size integers (removes extra space)
                    .allow_trailing_bytes(); // Prevents errors when decoding extra bytes
                println!("{}", hex::encode(config.serialize(&selected.genomes).unwrap()));

                self.blobs.push(blob);
            }
        }

        loop {
            self.draw();
            next_frame().await;
        }
    }

    pub async fn run_generation(&mut self, i: i16) {
        // just do 300 steps for now.
        for _ in 1..300 {
            self.step();

            //if i > 90 {
            if true {
                self.draw();
                next_frame().await;
            }
        }
    }

    /// Simulates one step in the environment.
    fn step(&mut self) {
        // Loop thru each blob and handle its neural network.
        for blob in self.blobs.iter_mut() {
            // TODO: might be smart to move this into genome + unprivatize.
            // something wrong here??? !!

            // Px = blob.x
            // Py = blob.y
            let random = rand::rng().random_range(-1. ..= 1.);
            // I0.. = blob.internal_state.I0..

            let mut mx = 0.;
            let mut my = 0.;
            let mut I0 = 0.;
            let mut I1 = 0.;
            let mut I2 = 0.;
            let mut I3 = 0.;

            for genome in blob.genomes {
                let source = match genome.source {
                    Source::Px => blob.x,
                    Source::Py => blob.y,
                    Source::Random => random,
                    Source::I0 => blob.internal_state.I0,
                    Source::I1 => blob.internal_state.I1,
                    Source::I2 => blob.internal_state.I2,
                    Source::I3 => blob.internal_state.I3,
                    _ => 0., // panic
                };

                let input_value = source * (genome.weight as f32);

                match genome.sink {
                    // assuming 128x128 board.
                    Sink::Mx => mx += input_value,
                    Sink::My => my += input_value,
                    Sink::I0 => I0 += input_value,
                    Sink::I1 => I1 += input_value,
                    Sink::I2 => I2 += input_value,
                    Sink::I3 => I3 += input_value,
                }
            }

            blob.x += Self::translate(blob.x, mx.tanh()) * (1./64.);
            blob.y += Self::translate(blob.y, my.tanh()) * (1./64.);
            blob.internal_state.I0 += I0.tanh().abs();
            blob.internal_state.I1 += I1.tanh();
            blob.internal_state.I2 += I2.tanh();
            blob.internal_state.I3 += I3.tanh();
        }
    }

    /// Returns either 0 or +/-1 based on probability as provided. Always
    /// 0 if passing borders.
    fn translate(position: f32, probability: f32) -> f32 {
        // If we want to go off the screen, return 0.
        if (position + (1./64.) * probability.signum()).abs() >= 1. {
            return 0.;
        }

        let mut rng = rand::rng();
        if rng.random_range(0. .. 1.) < probability.abs() {
            return 1. * probability.signum(); // direction
        } else {
            return 0.;
        }
    }

    fn draw(&self) {
        clear_background(BLACK);
        let scale = 4.0; // Scale factor to zoom in (1x1 becomes 4x4 pixels)

        for blob in &self.blobs {
            let (screen_x, screen_y) = self.to_screen_coords(blob.x, blob.y);
            draw_rectangle(screen_x, screen_y, scale, scale, WHITE);
        }
    }

    fn to_screen_coords(&self, x: f32, y: f32) -> (f32, f32) {
        let screen_x = ((x + 1.0) * 64.0) * 4.0; // Scale coordinates
        let screen_y = ((y + 1.0) * 64.0) * 4.0;
        (screen_x, screen_y)
    }
}
