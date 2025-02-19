pub mod blob;
pub mod genome;
pub mod neurons;
pub mod simulate;

use macroquad::prelude::*;

// TODO: consider setting up like guides, each step changes the code a bit and
// has its own directory.

#[macroquad::main(window_conf)]
async fn main() {
    // Let's make some random creatures.
    let _my_first_blob = blob::Blob::random_new();

    // Now let's make 100 of them.
    let mut blobs = vec![];
    for _ in 0..200 {
        blobs.push(blob::Blob::random_new());
    }

    // println!("{:#?}", blobs[5]);

    let mut sim = simulate::Simulator::new(blobs);
    sim.run().await;
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Simulator".to_owned(),
        window_width: 512 + 4,
        window_height: 512 + 4,
        fullscreen: true,
        ..Default::default()
    }
}
