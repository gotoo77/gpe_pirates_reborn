use gotoo_pixel_engine::{Framebuffer, Pixel};
use gpe_pirates_reborn::{
    app::{App, Controls},
    draw::{scene, ship},
    navigation::{HEIGHT, SCALE, SEA_HEIGHT, WIDTH, camera},
};
use std::time::Duration;

#[test]
fn oriented_ship_is_detailed_larger_than_physics_and_sails_change_silhouette() {
    let mut variants = Vec::new();
    for heading in 0..16 {
        let mut raised = Framebuffer::new(40, 40);
        raised.clear(Pixel::BLACK);
        let mut lowered = raised.clone();
        ship(&mut raised, 20, 20, heading, true);
        ship(&mut lowered, 20, 20, heading, false);
        assert!(raised != lowered);
        let mut count = 0;
        let mut beyond_physics = false;
        for y in 0..40 {
            for x in 0..40 {
                if raised.pixel(x, y) != Some(Pixel::BLACK) {
                    count += 1;
                    assert!((x - 20).abs() <= 16 && (y - 20).abs() <= 16);
                    beyond_physics |= (x - 20).abs() > 6 || (y - 20).abs() > 6;
                }
            }
        }
        assert!(count > 120 && beyond_physics);
        variants.push(raised);
    }
    for i in 0..16 {
        for j in 0..i {
            assert!(variants[i] != variants[j]);
        }
    }
}

#[test]
fn hud_stays_in_screen_space_and_pause_is_visible() {
    let mut app = App::default();
    let mut fb = Framebuffer::new(WIDTH, HEIGHT);
    scene(&mut fb, &app);
    let initial = fb.clone();
    app.advance(
        Duration::ZERO,
        Controls {
            pause: true,
            ..Controls::default()
        },
    );
    scene(&mut fb, &app);
    assert!(fb != initial);
    assert_eq!(fb.pixel(0, HEIGHT as i32 - 1), Some(Pixel::rgb(12, 30, 39)));
}

#[test]
fn whole_ship_remains_visible_at_every_world_corner_and_beneath_overlays() {
    let mut app = App::default();
    let mut fb = Framebuffer::new(WIDTH, HEIGHT);
    let background = Pixel::rgb(255, 0, 255);
    for pos in [[7, 7], [1593, 7], [7, 993], [1593, 993], [1500, 50]] {
        app.sim.position = pos.map(|v| v * SCALE);
        let cam = camera(app.sim.position);
        let x = ((app.sim.position[0] - cam[0] + SCALE / 2) / SCALE) as i32;
        let y = ((app.sim.position[1] - cam[1] + SCALE / 2) / SCALE) as i32;
        for heading in 0..16 {
            app.sim.heading = heading;
            scene(&mut fb, &app);
            let mut sprite = Framebuffer::new(40, 40);
            sprite.clear(background);
            ship(&mut sprite, 20, 20, heading, false);
            for sy in 0..40 {
                for sx in 0..40 {
                    let color = sprite.pixel(sx, sy).unwrap();
                    if color != background {
                        assert!((0..WIDTH as i32).contains(&(x + sx - 20)));
                        assert!((0..SEA_HEIGHT as i32).contains(&(y + sy - 20)));
                        assert_eq!(fb.pixel(x + sx - 20, y + sy - 20), Some(color));
                    }
                }
            }
        }
    }
}

#[test]
fn visual_animation_and_chart_never_mutate_simulation() {
    let mut app = App::default();
    let before = app.sim.clone();
    let mut fb = Framebuffer::new(WIDTH, HEIGHT);
    scene(&mut fb, &app);
    assert_eq!(before, app.sim);
    let initial = fb.clone();
    app.sim.ticks = 20;
    scene(&mut fb, &app);
    assert!(fb != initial);
    app.chart_open = true;
    let state = app.sim.clone();
    scene(&mut fb, &app);
    assert_eq!(app.sim, state);
}

#[test]
fn chart_ship_is_not_hidden_by_return_help_at_world_edges() {
    let mut app = App::default();
    app.chart_open = true;
    let mut fb = Framebuffer::new(WIDTH, HEIGHT);
    let map_h = (HEIGHT - gpe_pirates_reborn::navigation::HUD_HEIGHT - 56).min(560) as i32;
    let map_w = map_h * 8 / 5;
    let ox = (WIDTH as i32 - map_w) / 2;
    let background = Pixel::rgb(255, 0, 255);
    for [wx, wy] in [[7, 7], [1593, 993], [100, 975]] {
        app.sim.position = [wx * SCALE, wy * SCALE];
        let x = ox + (wx * i64::from(map_w) / 1600) as i32;
        let y = 38 + (wy * i64::from(map_h) / 1000) as i32;
        for heading in 0..16 {
            app.sim.heading = heading;
            scene(&mut fb, &app);
            let mut expected = Framebuffer::new(40, 40);
            expected.clear(background);
            ship(&mut expected, 20, 20, heading, false);
            for sy in 0..40 {
                for sx in 0..40 {
                    let color = expected.pixel(sx, sy).unwrap();
                    if color != background {
                        assert_eq!(fb.pixel(x + sx - 20, y + sy - 20), Some(color));
                    }
                }
            }
        }
    }
}
