//! Local CPU costs only; GPU presentation and human play are separate checks.
use gotoo_pixel_engine::Framebuffer;
use gpe_pirates_reborn::{
    app::App,
    draw,
    navigation::{Command, HEIGHT, Simulation, WIDTH},
};
use std::{hint::black_box, time::Instant};

fn main() {
    let mut sim = Simulation::default();
    let started = Instant::now();
    for tick in 0..100_000 {
        sim.tick(black_box(Command {
            rudder: if tick % 127 < 60 { 1 } else { -1 },
            raise: tick % 200 == 0,
            lower: tick % 391 == 390,
        }));
    }
    println!(
        "tick CPU: {:.3} us (100000 ticks)",
        started.elapsed().as_secs_f64() * 1_000_000.0 / 100_000.0
    );
    black_box(sim);
    let mut app = App::default();
    for (width, height) in [(WIDTH, HEIGHT), (1280, 720)] {
        let mut fb = Framebuffer::new(width, height);
        let cold = Instant::now();
        draw::scene(&mut fb, &app);
        println!(
            "first draw CPU: {:.3} ms ({width}x{height}, includes lazy art on first size)",
            cold.elapsed().as_secs_f64() * 1000.0
        );
        let started = Instant::now();
        for frame in 0..1000 {
            app.sim.ticks = frame;
            draw::scene(black_box(&mut fb), black_box(&app));
        }
        println!(
            "draw CPU: {:.3} ms (1000 frames, {width}x{height})",
            started.elapsed().as_secs_f64() * 1000.0 / 1000.0
        );
    }
}
