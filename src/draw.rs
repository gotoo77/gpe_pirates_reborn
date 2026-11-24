//! Original temporary pixel graphics; all solid shore pixels stay inside land.
use crate::{
    app::App,
    navigation::{
        HEADING_NAMES, HEADINGS, HEIGHT, ISLANDS, PORTS, SCALE, SEA_HEIGHT, WIDTH, WORLD, camera,
    },
};
use gotoo_pixel_engine::{Framebuffer, Pixel};

const SEA: Pixel = Pixel::rgb(23, 81, 108);
const WAVE: Pixel = Pixel::rgb(54, 113, 133);
const INK: Pixel = Pixel::rgb(16, 40, 56);
const PAPER: Pixel = Pixel::rgb(239, 228, 198);
const SAND: Pixel = Pixel::rgb(227, 204, 139);
const GREEN: Pixel = Pixel::rgb(87, 133, 74);
const WOOD: Pixel = Pixel::rgb(105, 75, 48);

pub fn ship(fb: &mut Framebuffer, x: i32, y: i32, heading: u8, sails: bool) {
    let [dx, dy] = HEADINGS[usize::from(heading)];
    let point = |forward: i64, side: i64| {
        (
            x + ((forward * dx - side * dy) / SCALE) as i32,
            y + ((forward * dy + side * dx) / SCALE) as i32,
        )
    };
    let hull = [(5, 0), (1, 4), (-4, 3), (-4, -3), (1, -4), (5, 0)];
    // Fill small hull with integer scan strokes, then draw a bright bow.
    for side in -2..=2 {
        let a = point(-3, side);
        let b = point(2, side);
        fb.draw_line(a.0, a.1, b.0, b.1, WOOD);
    }
    for pair in hull.windows(2) {
        let a = point(pair[0].0, pair[0].1);
        let b = point(pair[1].0, pair[1].1);
        fb.draw_line(a.0, a.1, b.0, b.1, SAND);
    }
    let a = point(-1, -3);
    let b = point(-1, 3);
    fb.draw_line(a.0, a.1, b.0, b.1, INK);
    if sails {
        for side in -3_i64..=3 {
            let a = point(0, side);
            let b = point(3 - side.abs(), side);
            fb.draw_line(a.0, a.1, b.0, b.1, PAPER);
        }
    } else {
        let a = point(-1, -1);
        let b = point(-1, 1);
        fb.draw_line(a.0, a.1, b.0, b.1, PAPER);
    }
    let a = point(3, 0);
    let b = point(5, 0);
    fb.draw_line(a.0, a.1, b.0, b.1, Pixel::WHITE);
}

fn label(fb: &mut Framebuffer, x: i32, y: i32, text: &str, color: Pixel) {
    fb.draw_text(x + 1, y + 1, text, INK);
    fb.draw_text(x, y, text, color);
}

pub fn scene(fb: &mut Framebuffer, app: &App) {
    let sim = &app.sim;
    let origin = camera(sim.position);
    // Round the camera only for drawing; simulation and camera retain subunits.
    let cx = ((origin[0] + SCALE / 2) / SCALE) as i32;
    let cy = ((origin[1] + SCALE / 2) / SCALE) as i32;
    fb.clear(SEA);
    for y in (0..WORLD[1] as i32).step_by(37) {
        if y - cy < -30 || y - cy >= SEA_HEIGHT as i32 {
            continue;
        }
        for x in (0..WORLD[0] as i32).step_by(47) {
            if x - cx < -40 || x - cx >= WIDTH as i32 {
                continue;
            }
            fb.draw_line(x + 7 - cx, y + 12 - cy, x + 13 - cx, y + 12 - cy, WAVE);
            fb.draw_line(x + 32 - cx, y + 27 - cy, x + 40 - cx, y + 27 - cy, WAVE);
        }
    }
    fb.draw_rect(-cx, -cy, WORLD[0] as u32, WORLD[1] as u32, SAND);
    for island in ISLANDS {
        let [x, y, w, h] = island.rect;
        let x = x as i32 - cx;
        let y = y as i32 - cy;
        fb.fill_rect(x, y, w as u32, h as u32, SAND);
        fb.fill_rect(x + 6, y + 6, w as u32 - 12, h as u32 - 12, GREEN);
        // A few trees, inland only; no extra collision geometry.
        for row in 0..h / 42 - 1 {
            for col in 0..w / 38 - 1 {
                let tx = x + 22 + col as i32 * 38 + (row % 2) as i32 * 7;
                let ty = y + 28 + row as i32 * 42;
                fb.fill_rect(tx + 2, ty + 5, 2, 5, WOOD);
                fb.fill_rect(tx, ty, 6, 7, Pixel::rgb(49, 99, 67));
            }
        }
        if x + 12 >= 0 && x + 12 < WIDTH as i32 && y + 12 >= 18 && y + 12 < (SEA_HEIGHT - 10) as i32
        {
            label(fb, x + 12, y + 12, island.name, PAPER);
        }
    }
    for (index, port) in PORTS.iter().enumerate() {
        let x = port.position[0] as i32 - cx;
        let y = port.position[1] as i32 - cy;
        if !(-60..WIDTH as i32 + 60).contains(&x) || !(-30..SEA_HEIGHT as i32 + 30).contains(&y) {
            continue;
        }
        fb.draw_circle(x, y, 24, WAVE);
        // Quays point to their islands but do not occupy the arrival disk.
        match index {
            0 => fb.fill_rect(x - 32, y - 3, 7, 6, WOOD),
            1 => fb.fill_rect(x + 25, y - 3, 7, 6, WOOD),
            _ => fb.fill_rect(x - 3, y + 25, 6, 7, WOOD),
        }
        fb.draw_rect(x - 3, y - 3, 7, 7, SAND);
        fb.draw_line(x, y - 9, x, y - 4, PAPER);
        fb.fill_rect(x + 1, y - 9, 4, 3, Pixel::rgb(163, 79, 57));
        let text_width = Framebuffer::text_size(port.name, 1).0 as i32;
        let label_x = (x - text_width / 2).clamp(2, WIDTH as i32 - text_width - 2);
        if (20..SEA_HEIGHT as i32 - 10).contains(&(y + 29)) {
            label(fb, label_x, y + 29, port.name, PAPER);
        }
    }
    let x = ((sim.position[0] + SCALE / 2) / SCALE) as i32 - cx;
    let y = ((sim.position[1] + SCALE / 2) / SCALE) as i32 - cy;
    // A compact wind vane. The arrow points WHERE the wind blows.
    fb.fill_circle(595, 50, 23, INK);
    fb.draw_circle(595, 50, 23, SAND);
    let [vx, vy] = HEADINGS[usize::from(sim.wind.heading)];
    let ex = 595 + (vx * 15 / SCALE) as i32;
    let ey = 50 + (vy * 15 / SCALE) as i32;
    fb.draw_line(
        595 - (vx * 15 / SCALE) as i32,
        50 - (vy * 15 / SCALE) as i32,
        ex,
        ey,
        PAPER,
    );
    for side in [-1, 1] {
        fb.draw_line(
            ex,
            ey,
            ex + ((-vx * 6 - vy * 4 * side) / SCALE) as i32,
            ey + ((-vy * 6 + vx * 4 * side) / SCALE) as i32,
            PAPER,
        );
    }
    label(fb, 582, 18, "VENT", PAPER);
    // Keep the whole 312-row world area usable, including the north edge.
    // The ship is drawn last so the vane cannot hide it near a world corner.
    ship(fb, x, y, sim.heading, sim.sails);
    fb.fill_rect(0, SEA_HEIGHT as i32, WIDTH, HEIGHT - SEA_HEIGHT, INK);
    fb.draw_line(
        0,
        SEA_HEIGHT as i32,
        WIDTH as i32 - 1,
        SEA_HEIGHT as i32,
        SAND,
    );
    let target = if sim.sails {
        crate::navigation::speed(sim.heading, sim.wind)
    } else {
        0
    };
    fb.draw_text(
        8,
        318,
        &format!(
            "CAP {}  VIT {:.1}  CIBLE {:.1}  VOILES {}",
            HEADING_NAMES[usize::from(sim.heading)],
            sim.effective_speed as f32 / 100.0,
            target as f32 / 100.0,
            if sim.sails { "HAUTES" } else { "BASSES" }
        ),
        PAPER,
    );
    fb.draw_text(
        8,
        329,
        &format!(
            "VENT VERS {} / FORCE {}",
            HEADING_NAMES[usize::from(sim.wind.heading)],
            sim.wind.force
        ),
        SAND,
    );
    let (nearest, dist) = PORTS
        .iter()
        .enumerate()
        .map(|(index, port)| {
            let dx = port.position[0] * SCALE - sim.position[0];
            let dy = port.position[1] * SCALE - sim.position[1];
            (index, (dx * dx + dy * dy) as u64)
        })
        .min_by_key(|(_, distance)| *distance)
        .expect("three ports");
    let delta = [
        PORTS[nearest].position[0] * SCALE - sim.position[0],
        PORTS[nearest].position[1] * SCALE - sim.position[1],
    ];
    let bearing = HEADINGS
        .iter()
        .enumerate()
        .max_by_key(|(_, h)| h[0] * delta[0] + h[1] * delta[1])
        .map_or(0, |(i, _)| i);
    fb.draw_text(
        226,
        329,
        &format!(
            "PROCHE {} {} {}u",
            PORTS[nearest].name,
            if dist == 0 {
                "ICI"
            } else {
                HEADING_NAMES[bearing]
            },
            dist.isqrt() / SCALE as u64
        ),
        PAPER,
    );
    let status = if app.paused {
        "PAUSE".to_owned()
    } else if sim.blocked {
        "TERRE - TOURNER POUR REPARTIR".to_owned()
    } else if let Some(port) = sim.arrival {
        format!("ARRIVEE {}", PORTS[port].name)
    } else {
        format!("DERNIER {}", PORTS[sim.last_port].name)
    };
    fb.draw_text(8, 340, &status, SAND);
    fb.draw_text(
        8,
        351,
        "GAUCHE/DROITE BARRE | HAUT/BAS VOILES | ECHAP PAUSE | R RESET",
        PAPER,
    );
}
