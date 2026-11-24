use gotoo_pixel_engine::{Framebuffer, Pixel};
use gpe_pirates_reborn::{
    app::{App, Controls},
    draw::{scene, ship},
    navigation::{HEIGHT, WIDTH},
};
use std::time::Duration;

#[test]
fn oriented_ship_fits_collision_square_and_sails_change_silhouette() {
    for heading in 0..16 {
        let mut raised = Framebuffer::new(32, 32);
        raised.clear(Pixel::BLACK);
        let mut lowered = Framebuffer::new(32, 32);
        lowered.clear(Pixel::BLACK);
        ship(&mut raised, 16, 16, heading, true);
        ship(&mut lowered, 16, 16, heading, false);
        assert_ne!(raised.as_rgba8(), lowered.as_rgba8());
        let mut count = 0;
        for y in 0..32 {
            for x in 0..32 {
                if raised.pixel(x, y) != Some(Pixel::BLACK) {
                    count += 1;
                    assert!((x - 16).abs() <= 6 && (y - 16).abs() <= 6);
                }
            }
        }
        assert!(count > 15);
    }
}

#[test]
fn hud_stays_in_screen_space_and_pause_is_visible() {
    let mut app = App::default();
    let mut fb = Framebuffer::new(WIDTH, HEIGHT);
    scene(&mut fb, &app);
    let initial = fb.as_rgba8().to_vec();
    app.advance(
        Duration::ZERO,
        Controls {
            pause: true,
            ..Controls::default()
        },
    );
    scene(&mut fb, &app);
    assert_ne!(fb.as_rgba8(), initial);
    assert_eq!(fb.pixel(0, 359), Some(Pixel::rgb(16, 40, 56)));
    assert_eq!(fb.pixel(639, 311), Some(Pixel::rgb(23, 81, 108)));
}

#[test]
fn ship_remains_visible_at_north_edge_and_beneath_wind_overlay() {
    let mut app = App::default();
    let mut fb = Framebuffer::new(WIDTH, HEIGHT);
    app.sim.position = [800 * 1024, 7 * 1024];
    scene(&mut fb, &app);
    assert_eq!(fb.pixel(325, 7), Some(Pixel::WHITE));
    app.sim.position = [1555 * 1024, 50 * 1024];
    scene(&mut fb, &app);
    assert_eq!(fb.pixel(600, 50), Some(Pixel::WHITE));
}
