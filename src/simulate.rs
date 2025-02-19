use crate::blob::*;
use crate::neurons::*;

use macroquad::prelude::*;

/// The simulation environment.
pub struct Simulator {
    blobs: Vec<Blob>,
}

// TODO: how to multithread this?
// TODO: generations?

impl Simulator {
    pub fn new(blobs: Vec<Blob>) -> Self {
        Self { blobs }
    }

    pub async fn run(&mut self) {
        println!("{:#?}", self.blobs[1]);

        // just do 300 steps for now.
        for _ in 1..300 {
            self.step();
            self.draw();
            next_frame().await;
        }

        println!("{:#?}", self.blobs[1]);

        loop {
            self.draw();
            next_frame().await;
        }
    }

    /// Simulates one step in the environment.
    fn step(&mut self) {
        // Loop thru each blob and handle its neural network.
        for blob in self.blobs.iter_mut() {
            // TODO: might be smart to move this into genome + unprivatize.
            for genome in blob.genomes {
                let source = match genome.source {
                    Source::Px => blob.x,
                    Source::Py => blob.y,
                    _ => 0., // panic
                };

                match genome.sink {
                    // assuming 128x128 board.
                    Sink::Mx => blob.x = f32::clamp(blob.x + source * (genome.weight as f32) * (1. / 64.), -1., 1.),
                    Sink::My => blob.y = f32::clamp(blob.y + source * (genome.weight as f32) * (1. / 64.), -1., 1.),
                    _ => (), // panic
                }
            }
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
