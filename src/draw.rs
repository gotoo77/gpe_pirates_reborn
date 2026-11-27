//! Original, code-drawn maritime pixel art. No presentation value enters physics.
use crate::{
    app::App,
    economy::{CAPACITY, GOODS, Side},
    navigation::{
        HEADING_NAMES, HEADINGS, HUD_HEIGHT, ISLANDS, PORTS, SCALE, WORLD, camera_for,
        land_contains,
    },
};
use gotoo_pixel_engine::{Framebuffer, Image, Pixel};
use std::sync::OnceLock;

const SEA: Pixel = Pixel::rgb(12, 66, 89);
const INK: Pixel = Pixel::rgb(12, 30, 39);
const PANEL: Pixel = Pixel::rgb(20, 43, 51);
const PAPER: Pixel = Pixel::rgb(248, 235, 198);
const GOLD: Pixel = Pixel::rgb(206, 161, 83);
const MUTED: Pixel = Pixel::rgb(119, 162, 168);
const FOAM: Pixel = Pixel::rgb(156, 219, 205);
const RED: Pixel = Pixel::rgb(189, 66, 45);
const WOOD: Pixel = Pixel::rgb(93, 52, 35);

// A stable texture hash, not procedural geography or simulation randomness.
fn grain(x: i32, y: i32) -> u32 {
    let v = (x as u32).wrapping_mul(0x45d9f3b) ^ (y as u32).wrapping_mul(0x119de1f3);
    (v ^ (v >> 16)).wrapping_mul(0x45d9f3b)
}

fn label(fb: &mut Framebuffer, x: i32, y: i32, text: &str, color: Pixel) {
    fb.draw_text(x + 1, y + 1, text, INK);
    fb.draw_text(x, y, text, color);
}

fn local_ship(f: i32, s: i32, sails: bool) -> Option<Pixel> {
    let half = match f {
        -12 => 2,
        -11..=-9 => 4,
        -8..=4 => 5,
        5..=7 => 4,
        8..=10 => 3,
        11 => 2,
        12..=13 => 0,
        _ => -1,
    };
    let mut color = if s.abs() <= half {
        Some(if s.abs() == half || f == -12 {
            GOLD
        } else if s.abs() == half - 1 {
            WOOD
        } else if f < -7 {
            Pixel::rgb(119, 58, 39)
        } else if (f + s).rem_euclid(4) == 0 {
            Pixel::rgb(171, 113, 60)
        } else {
            Pixel::rgb(144, 88, 47)
        })
    } else {
        None
    };
    // Two transverse yards and curved ivory sails, read from every heading.
    for mast in [-5, 4] {
        if f == mast && s.abs() <= 7 {
            color = Some(WOOD);
        }
        if sails && s.abs() <= 6 && f > mast && f <= mast + 3 - (s.abs() / 3) {
            color = Some(if s < -3 || f == mast + 1 {
                Pixel::rgb(200, 185, 150)
            } else {
                PAPER
            });
        }
        if !sails && f == mast + 1 && s.abs() <= 3 {
            color = Some(PAPER);
        }
        if f == mast && s == 0 {
            color = Some(INK);
        }
    }
    if (-10..=-7).contains(&f) && s == -6 {
        color = Some(RED);
    }
    if f == 13 && s == 0 {
        color = Some(Pixel::WHITE);
    }
    color
}

/// 26-pixel hull, two masts, scarlet pennant. Physical hull remains ±6.
pub fn ship(fb: &mut Framebuffer, x: i32, y: i32, heading: u8, sails: bool) {
    let [hx, hy] = HEADINGS[usize::from(heading)];
    for dy in -16..=16 {
        for dx in -16..=16 {
            let round = |v: i64| {
                if v >= 0 {
                    (v + SCALE / 2) / SCALE
                } else {
                    (v - SCALE / 2) / SCALE
                }
            };
            let f = round(i64::from(dx) * hx + i64::from(dy) * hy) as i32;
            let s = round(-i64::from(dx) * hy + i64::from(dy) * hx) as i32;
            if let Some(color) = local_ship(f, s, sails) {
                fb.draw(x + dx, y + dy, color);
            }
        }
    }
}

fn palm(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.fill_rect(x - 5, y + 8, 17, 4, Pixel::rgb(48, 99, 54));
    fb.draw_line(x + 2, y + 8, x, y - 4, WOOD);
    fb.draw_line(x + 3, y + 8, x + 1, y - 4, GOLD);
    for (dx, dy) in [
        (-10, -1),
        (-8, -6),
        (0, -9),
        (8, -6),
        (11, 0),
        (6, 4),
        (-6, 3),
    ] {
        fb.draw_line(x, y - 4, x + dx, y + dy, Pixel::rgb(29, 92, 57));
        fb.draw_line(x, y - 5, x + dx, y + dy - 2, Pixel::rgb(82, 145, 57));
        fb.draw_line(x, y - 6, x + dx / 2, y + dy - 3, Pixel::rgb(143, 173, 65));
    }
    fb.draw(x, y - 4, GOLD);
}

const TOWNS: [[(i32, i32, bool); 3]; 3] = [
    [(248, 240, false), (230, 260, true), (192, 300, false)],
    [(1141, 258, true), (1166, 273, false), (1146, 300, false)],
    [(713, 698, false), (741, 708, true), (765, 724, false)],
];

struct IslandArt {
    image: Image,
    origin: [i32; 2],
}
static ART: OnceLock<[IslandArt; 4]> = OnceLock::new();

fn island_art() -> &'static [IslandArt; 4] {
    ART.get_or_init(|| {
        std::array::from_fn(|i| {
            let [x, y, w, h] = ISLANDS[i].rect;
            let (w, h) = ((w + 32) as usize, (h + 32) as usize);
            let origin = [x as i32 - 16, y as i32 - 16];
            let mut mask = vec![false; w * h];
            for yy in 0..h {
                for xx in 0..w {
                    mask[yy * w + xx] = land_contains([
                        i64::from(origin[0]) + xx as i64,
                        i64::from(origin[1]) + yy as i64,
                    ]);
                }
            }
            // Manhattan distance to the opposite material: beaches and shallows
            // stay on their own side of the exact polygon silhouette.
            let mut distance = vec![1000_i32; w * h];
            for yy in 1..h - 1 {
                for xx in 1..w - 1 {
                    let p = yy * w + xx;
                    if [p - 1, p + 1, p - w, p + w]
                        .iter()
                        .any(|&q| mask[p] != mask[q])
                    {
                        distance[p] = 0;
                    }
                }
            }
            for yy in 1..h {
                for xx in 1..w {
                    let p = yy * w + xx;
                    distance[p] = distance[p]
                        .min(distance[p - 1] + 1)
                        .min(distance[p - w] + 1);
                }
            }
            for yy in (0..h - 1).rev() {
                for xx in (0..w - 1).rev() {
                    let p = yy * w + xx;
                    distance[p] = distance[p]
                        .min(distance[p + 1] + 1)
                        .min(distance[p + w] + 1);
                }
            }
            let mut fb = Framebuffer::new(w as u32, h as u32);
            for yy in 0..h {
                for xx in 0..w {
                    let p = yy * w + xx;
                    let d = distance[p];
                    let wx = origin[0] + xx as i32;
                    let wy = origin[1] + yy as i32;
                    let g = grain(wx / 3, wy / 2);
                    let color = if mask[p] {
                        if d < 2 {
                            Pixel::rgb(238, 222, 163)
                        } else if d < 7 + (g % 3) as i32 {
                            Pixel::rgb(211 + (g % 13) as u8, 191 + (g % 12) as u8, 124)
                        } else if (wx / 17 + wy / 23 + (g % 3) as i32).rem_euclid(5) == 0 {
                            Pixel::rgb(115, 151, 64)
                        } else if g % 7 < 2 {
                            Pixel::rgb(77, 124, 56)
                        } else {
                            Pixel::rgb(93, 139, 59)
                        }
                    } else {
                        match d {
                            0 => FOAM,
                            1..=3 => Pixel::rgb(35, 147, 143),
                            4..=7 => Pixel::rgb(22, 112, 126),
                            8..=11 => Pixel::rgb(16, 85, 109),
                            _ => Pixel::rgba(0, 0, 0, 0),
                        }
                    };
                    fb.draw(xx as i32, yy as i32, color);
                }
            }
            // Inland rock outcrops. Every decorative footprint is checked against
            // land; no floating trees or visually solid rock outside collisions.
            for k in 0..45 {
                let g = grain(k, i as i32 + 11);
                let tx = 18 + (g % (w as u32 - 36)) as i32;
                let ty = 22 + ((g >> 12) % (h as u32 - 44)) as i32;
                let on_land = [[-14, -15], [14, -15], [-14, 14], [14, 14], [0, 0]]
                    .iter()
                    .all(|[dx, dy]| {
                        let xx = tx + dx;
                        let yy = ty + dy;
                        mask[yy as usize * w + xx as usize]
                            && distance[yy as usize * w + xx as usize] > 8
                    });
                if !on_land {
                    continue;
                }
                if k % 4 == 0 {
                    for row in 0..14 {
                        fb.draw_line(
                            tx - row / 2,
                            ty - row,
                            tx + row / 2 + 3,
                            ty - row,
                            Pixel::rgb(103, 101, 76),
                        );
                    }
                    fb.draw_line(tx, ty - 13, tx - 5, ty, Pixel::rgb(183, 164, 113));
                    fb.draw_line(tx + 1, ty - 11, tx + 5, ty - 1, Pixel::rgb(74, 86, 68));
                } else {
                    palm(&mut fb, tx, ty);
                }
            }
            if i < TOWNS.len() {
                for &(x, y, tower) in &TOWNS[i] {
                    building(&mut fb, x - origin[0], y - origin[1], tower);
                }
            }
            IslandArt {
                image: Image::from_rgba8(w as u32, h as u32, fb.as_rgba8().to_vec())
                    .expect("fixed island image"),
                origin,
            }
        })
    })
}

fn ocean(fb: &mut Framebuffer, cx: i32, cy: i32, sea_h: i32, ticks: u64) {
    fb.clear(SEA);
    let phase = (ticks / 8) as i32;
    // Jittered, world-anchored streaks, with independent phases and lengths.
    for gy in (cy - 40).div_euclid(31)..=(cy + sea_h + 40).div_euclid(31) {
        for gx in (cx - 50).div_euclid(43)..=(cx + fb.width() as i32 + 50).div_euclid(43) {
            let g = grain(gx, gy);
            let x = gx * 43 + (g % 29) as i32 - cx;
            let y = gy * 31 + ((g >> 8) % 23) as i32 - cy;
            let drift = (phase + (g >> 16) as i32).rem_euclid(12);
            for (offset, length, color) in [
                (0, 12, Pixel::rgb(14, 76, 99)),
                (13, 7, Pixel::rgb(17, 82, 104)),
                (25, 5, Pixel::rgb(18, 90, 109)),
            ] {
                fb.draw_line(
                    x + offset,
                    y + offset / 3,
                    x + offset + length,
                    y + offset / 3,
                    color,
                );
            }
            if g.is_multiple_of(5) && drift < 8 {
                let yy = y + 12 + drift / 4;
                fb.draw_line(
                    x + drift,
                    yy,
                    x + drift + 3 + (g % 6) as i32,
                    yy,
                    Pixel::rgb(44, 121, 137),
                );
                fb.draw(x + drift + 2, yy - 1, Pixel::rgb(29, 102, 121));
            }
        }
    }
}

fn building(fb: &mut Framebuffer, x: i32, y: i32, tower: bool) {
    let h = if tower { 24 } else { 12 };
    fb.fill_rect(x + 3, y + 4, 17, h + 2, Pixel::rgb(53, 88, 57));
    fb.fill_rect(x, y, 16, h, Pixel::rgb(229, 211, 166));
    fb.fill_rect(x + 11, y, 5, h, Pixel::rgb(172, 152, 112));
    for row in 0..5 {
        fb.draw_line(
            x - 2 + row,
            y - 5 + row,
            x + 17 - row,
            y - 5 + row,
            if row % 2 == 0 {
                RED
            } else {
                Pixel::rgb(142, 53, 37)
            },
        );
    }
    fb.fill_rect(x + 6, y + h as i32 - 6, 4, 6, WOOD);
    fb.fill_rect(x + 2, y + 3, 3, 3, INK);
    fb.draw(x + 2, y + 3, GOLD);
    if tower {
        fb.fill_rect(x + 6, y + 3, 4, 6, INK);
        fb.draw_line(x + 8, y - 12, x + 8, y - 6, GOLD);
        fb.fill_rect(x + 9, y - 12, 6, 3, RED);
    }
}

fn ports(fb: &mut Framebuffer, cx: i32, cy: i32, sea_h: i32, ticks: u64) {
    // Original clusters placed on land; piers are navigation markers, as in P1.
    for (i, port) in PORTS.iter().enumerate() {
        let x = port.position[0] as i32 - cx;
        let y = port.position[1] as i32 - cy;
        if x < -120 || x > fb.width() as i32 + 120 || y < -100 || y > sea_h + 100 {
            continue;
        }
        match i {
            0 => {
                fb.fill_rect(x - 48, y - 4, 23, 8, WOOD);
                for dx in (-48..-25).step_by(4) {
                    fb.draw_line(x + dx, y - 3, x + dx, y + 2, GOLD);
                }
            }
            1 => {
                fb.fill_rect(x + 25, y - 4, 30, 8, WOOD);
                for dx in (25..55).step_by(4) {
                    fb.draw_line(x + dx, y - 3, x + dx, y + 2, GOLD);
                }
            }
            _ => {
                fb.fill_rect(x - 4, y + 26, 8, 31, WOOD);
                for dy in (26..57).step_by(4) {
                    fb.draw_line(x - 3, y + dy, x + 2, y + dy, GOLD);
                }
            }
        }
        // Broken approach ring and alternating harbour lights, on water.
        for h in (0..16).step_by(2) {
            let [vx, vy] = HEADINGS[h];
            let bx = x + (vx * 24 / SCALE) as i32;
            let by = y + (vy * 24 / SCALE) as i32;
            fb.fill_rect(
                bx,
                by,
                2,
                2,
                if (ticks / 30 + h as u64).is_multiple_of(3) {
                    PAPER
                } else {
                    MUTED
                },
            );
        }
        if (10..sea_h - 18).contains(&(y + 31)) {
            let tw = Framebuffer::text_size(port.name, 1).0 as i32;
            label(
                fb,
                (x - tw / 2).clamp(4, fb.width() as i32 - tw - 4),
                y + 31,
                port.name,
                PAPER,
            );
        }
    }
}

fn wake(fb: &mut Framebuffer, app: &App, cx: i32, cy: i32) {
    for &(p, tick, heading) in &app.wake {
        let age = app.sim.ticks - tick;
        let x = (p[0] / SCALE) as i32 - cx;
        let y = (p[1] / SCALE) as i32 - cy;
        let color = if age < 22 {
            FOAM
        } else if age < 50 {
            Pixel::rgb(56, 145, 151)
        } else {
            Pixel::rgb(26, 101, 119)
        };
        let radius = 2 + (age / 18) as i32;
        let [dx, dy] = HEADINGS[usize::from(heading)];
        for side in [-1, 1] {
            let px = x + (-dy * i64::from(radius) * side / SCALE) as i32;
            let py = y + (dx * i64::from(radius) * side / SCALE) as i32;
            fb.draw_line(
                px,
                py,
                px + (dx * 2 / SCALE) as i32,
                py + (dy * 2 / SCALE) as i32,
                color,
            );
        }
    }
}

fn compass(fb: &mut Framebuffer, x: i32, y: i32, heading: u8) {
    fb.fill_circle(x, y, 27, INK);
    fb.draw_circle(x, y, 26, GOLD);
    fb.draw_circle(x, y, 23, Pixel::rgb(71, 87, 78));
    for (i, &[dx, dy]) in HEADINGS.iter().enumerate() {
        let r = if i % 4 == 0 { 15 } else { 20 };
        fb.draw_line(
            x + (dx * r / SCALE) as i32,
            y + (dy * r / SCALE) as i32,
            x + (dx * 22 / SCALE) as i32,
            y + (dy * 22 / SCALE) as i32,
            GOLD,
        );
    }
    fb.draw_text(x - 2, y - 20, "N", PAPER);
    fb.draw_text(x - 2, y + 14, "S", MUTED);
    fb.draw_text(x + 15, y - 3, "E", MUTED);
    fb.draw_text(x - 20, y - 3, "O", MUTED);
    let [dx, dy] = HEADINGS[usize::from(heading)];
    let ex = x + (dx * 13 / SCALE) as i32;
    let ey = y + (dy * 13 / SCALE) as i32;
    fb.draw_line(
        x - (dx * 10 / SCALE) as i32,
        y - (dy * 10 / SCALE) as i32,
        ex,
        ey,
        PAPER,
    );
    for s in [-1, 1] {
        fb.draw_line(
            ex,
            ey,
            ex + ((-dx * 5 - dy * 3 * s) / SCALE) as i32,
            ey + ((-dy * 5 + dx * 3 * s) / SCALE) as i32,
            RED,
        );
    }
    fb.fill_circle(x, y, 2, GOLD);
}

fn hud(fb: &mut Framebuffer, app: &App) {
    let w = fb.width() as i32;
    let y = fb.height() as i32 - HUD_HEIGHT as i32;
    let sim = &app.sim;
    fb.fill_rect(0, y, fb.width(), HUD_HEIGHT, INK);
    fb.draw_line(0, y, w - 1, y, GOLD);
    fb.draw_line(0, y + 2, w - 1, y + 2, WOOD);
    let sections = [14, 145, 295, 470, 650];
    for x in sections.iter().skip(1) {
        fb.draw_line(*x - 14, y + 12, *x - 14, y + 49, Pixel::rgb(57, 76, 73));
    }
    fb.draw_text(14, y + 10, "CAP", MUTED);
    fb.draw_text_scaled(
        14,
        y + 25,
        HEADING_NAMES[usize::from(sim.heading)],
        2,
        PAPER,
    );
    fb.draw_text(145, y + 10, "VITESSE / u/s", MUTED);
    fb.draw_text_scaled(
        145,
        y + 25,
        &format!("{:.1}", sim.effective_speed as f32 / 100.0),
        2,
        PAPER,
    );
    fb.draw_text(295, y + 10, "VOILURE", MUTED);
    fb.draw_text_scaled(
        295,
        y + 25,
        if sim.sails { "HISSEE" } else { "BAISSEE" },
        2,
        PAPER,
    );
    fb.draw_text(470, y + 10, "VENT VERS", MUTED);
    fb.draw_text_scaled(
        470,
        y + 25,
        &format!(
            "{}  F{}",
            HEADING_NAMES[usize::from(sim.wind.heading)],
            sim.wind.force
        ),
        2,
        PAPER,
    );
    for i in 0..3 {
        fb.fill_rect(
            561 + i * 10,
            y + 11,
            6,
            5,
            if i < i32::from(sim.wind.force) {
                GOLD
            } else {
                PANEL
            },
        );
    }
    let (nearest, dist) = PORTS
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let dx = p.position[0] * SCALE - sim.position[0];
            let dy = p.position[1] * SCALE - sim.position[1];
            (i, (dx * dx + dy * dy) as u64)
        })
        .min_by_key(|(_, d)| *d)
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
    fb.draw_text(650, y + 10, "PORT PROCHE", MUTED);
    fb.draw_text(650, y + 25, PORTS[nearest].name, PAPER);
    fb.draw_text(
        650,
        y + 38,
        &format!(
            "{}  {} u",
            if dist == 0 {
                "ICI"
            } else {
                HEADING_NAMES[bearing]
            },
            dist.isqrt() / SCALE as u64
        ),
        GOLD,
    );
    compass(fb, w - 43, y + 31, sim.wind.heading);
    let status = if app.chart_open {
        "CARTE - NAVIGATION SUSPENDUE".to_owned()
    } else if app.paused {
        "PAUSE - ECHAP / START POUR REPRENDRE".to_owned()
    } else if app.awaiting_neutral() {
        "RELACHER BARRE / VOILES POUR COMMANDER".to_owned()
    } else if sim.blocked {
        "TERRE - TOURNER POUR REPARTIR".to_owned()
    } else if let Some(i) = app.docked_port() {
        format!("{} - ENTREE / SOUTH : MARCHE", PORTS[i].name)
    } else {
        format!("DERNIER PORT : {}", PORTS[sim.last_port].name)
    };
    fb.draw_text(14, y + 55, &status, GOLD);
    fb.draw_text(
        650,
        y + 55,
        &format!(
            "OR {} | CALE {}/{} | J{}",
            app.economy.cash(),
            app.economy.used(),
            CAPACITY,
            app.economy.day()
        ),
        GOLD,
    );
    fb.draw_text(
        14,
        y + 73,
        if app.chart_open {
            "M / WEST / ECHAP : RETOUR | RELACHER BARRE / VOILES"
        } else {
            "FLECHES BARRE/VOILES | ECHAP PAUSE | M CARTE | R RESET"
        },
        PAPER,
    );
    fb.draw_text(650, y + 73, "PAD: D-PAD | WEST CARTE", MUTED);
}

fn chart(fb: &mut Framebuffer, app: &App) {
    let available_h = fb.height() as i32 - HUD_HEIGHT as i32 - 56;
    let map_h = available_h.min(560);
    let map_w = map_h * 8 / 5;
    let ox = (fb.width() as i32 - map_w) / 2;
    let oy = 38;
    let project = |p: [i64; 2]| {
        [
            ox + (p[0] * i64::from(map_w) / WORLD[0]) as i32,
            oy + (p[1] * i64::from(map_h) / WORLD[1]) as i32,
        ]
    };
    fb.clear(INK);
    fb.fill_rect(ox, oy, map_w as u32, map_h as u32, SEA);
    for x in (ox..ox + map_w).step_by(64) {
        fb.draw_line(x, oy, x, oy + map_h, Pixel::rgb(20, 78, 96));
    }
    for y in (oy..oy + map_h).step_by(40) {
        fb.draw_line(ox, y, ox + map_w, y, Pixel::rgb(20, 78, 96));
    }
    // Sample the same fixed terrain art, including the union coastline: no
    // false seams between overlapping lobes and no second map geometry.
    for art in island_art() {
        let [left, top] = project(art.origin.map(i64::from));
        let [right, bottom] = project([
            i64::from(art.origin[0]) + i64::from(art.image.width()),
            i64::from(art.origin[1]) + i64::from(art.image.height()),
        ]);
        for y in top..bottom {
            for x in left..right {
                let sx = ((x - left) * art.image.width() as i32 / (right - left)) as usize;
                let sy = ((y - top) * art.image.height() as i32 / (bottom - top)) as usize;
                let idx = (sy * art.image.width() as usize + sx) * 4;
                let rgba = &art.image.as_rgba8()[idx..idx + 4];
                if rgba[3] != 0 {
                    fb.draw(x, y, Pixel::rgba(rgba[0], rgba[1], rgba[2], rgba[3]));
                }
            }
        }
    }
    fb.draw_rect(ox - 1, oy - 1, map_w as u32 + 2, map_h as u32 + 2, GOLD);
    label(fb, ox, 16, "ARCHIPEL DES TROIS VENTS", PAPER);
    label(fb, ox + map_w - 119, 16, "CARTE / PAUSE", GOLD);
    for port in PORTS {
        let [x, y] = project(port.position);
        fb.fill_circle(x, y, 3, GOLD);
        label(fb, x - 26, y + 10, port.name, PAPER);
    }
    let p = project(app.sim.position.map(|v| v / SCALE));
    fb.draw_circle(p[0], p[1], 17, PAPER);
    ship(fb, p[0], p[1], app.sim.heading, app.sim.sails);
}

fn price(value: Option<u64>) -> String {
    value.map_or_else(|| "--".to_owned(), |v| v.to_string())
}

fn market(fb: &mut Framebuffer, app: &App, port: usize) {
    let e = &app.economy;
    fb.clear(INK);
    fb.fill_rect(20, 20, 920, 500, PANEL);
    fb.draw_rect(20, 20, 920, 500, GOLD);
    fb.draw_text_scaled(36, 36, PORTS[port].name, 3, PAPER);
    fb.draw_text_scaled(
        36,
        68,
        &format!(
            "OR {}     CALE {}/{}     JOUR {}",
            e.cash(),
            e.used(),
            CAPACITY,
            e.day()
        ),
        2,
        GOLD,
    );
    for (x, name) in [
        (52, "MARCHANDISE"),
        (270, "STOCK"),
        (390, "ACHAT/U"),
        (545, "VENTE/U"),
        (700, "EN CALE"),
    ] {
        fb.draw_text_scaled(x, 104, name, 2, MUTED);
    }
    let cargo = e.cargo();
    for (i, name) in GOODS.iter().enumerate() {
        let y = 132 + i as i32 * 36;
        if i == app.market_good {
            fb.fill_rect(36, y - 7, 888, 32, SEA);
            fb.draw_rect(36, y - 7, 888, 32, GOLD);
        }
        fb.draw_text_scaled(52, y, name, 2, PAPER);
        fb.draw_text_scaled(
            270,
            y,
            &e.stock(port, i).unwrap_or_default().to_string(),
            2,
            PAPER,
        );
        fb.draw_text_scaled(390, y, &price(e.marginal(port, i, Side::Buy)), 2, PAPER);
        fb.draw_text_scaled(545, y, &price(e.marginal(port, i, Side::Sell)), 2, PAPER);
        fb.draw_text_scaled(700, y, &cargo[i].to_string(), 2, PAPER);
    }
    fb.draw_line(36, 239, 924, 239, GOLD);
    fb.draw_text_scaled(
        36,
        251,
        &format!(
            "{} {} X {}",
            if app.market_side == Side::Buy {
                "ACHETER"
            } else {
                "VENDRE"
            },
            GOODS[app.market_good],
            app.market_quantity
        ),
        2,
        PAPER,
    );
    match app.market_quote() {
        Ok(q) => {
            let after = match app.market_side {
                Side::Buy => e.cash() - q.total(),
                Side::Sell => e.cash() + q.total(),
            };
            fb.draw_text_scaled(
                36,
                277,
                &format!("TOTAL {} OR    APRES : {} OR", q.total(), after),
                2,
                GOLD,
            );
        }
        Err(error) => {
            fb.draw_text_scaled(36, 277, error.message(), 2, RED);
        }
    }
    fb.draw_text(36, 305, &app.market_feedback, PAPER);
    fb.draw_line(36, 323, 924, 323, GOLD);
    fb.draw_text_scaled(36, 334, "DERNIERS PRIX OBSERVES (ACHAT/VENTE)", 2, MUTED);
    for (x, name) in [(360, "SUCRE"), (520, "BOIS"), (680, "OUTILS")] {
        fb.draw_text(x, 359, name, MUTED);
    }
    for (i, p) in PORTS.iter().enumerate() {
        let y = 378 + i as i32 * 25;
        fb.draw_text(36, y, p.name, PAPER);
        if let Some(o) = e.observation(i) {
            fb.draw_text(180, y, &format!("IL Y A {} J", e.day() - o.day), MUTED);
            for g in 0..GOODS.len() {
                fb.draw_text_scaled(
                    360 + g as i32 * 160,
                    y - 3,
                    &format!("{}/{}", price(o.buy[g]), price(o.sell[g])),
                    2,
                    PAPER,
                );
            }
        } else {
            fb.draw_text(180, y, "NON VISITE", MUTED);
        }
    }
    fb.draw_line(36, 451, 924, 451, GOLD);
    fb.draw_text(
        36,
        466,
        "HAUT/BAS : BIEN  |  GAUCHE/DROITE : QUANTITE  |  TAB / NORTH : ACHAT/VENTE",
        PAPER,
    );
    fb.draw_text(
        36,
        487,
        "ENTREE / SOUTH : CONFIRMER  |  ECHAP / START / EAST : FERMER  |  R / SELECT : RESET",
        PAPER,
    );
}

pub fn scene(fb: &mut Framebuffer, app: &App) {
    if let Some(port) = app.market_port {
        market(fb, app, port);
        return;
    }
    if app.chart_open {
        chart(fb, app);
        hud(fb, app);
        return;
    }
    let sea_h = fb.height() - HUD_HEIGHT;
    let origin = camera_for(app.sim.position, fb.width(), sea_h);
    let cx = ((origin[0] + SCALE / 2).div_euclid(SCALE)) as i32;
    let cy = ((origin[1] + SCALE / 2).div_euclid(SCALE)) as i32;
    ocean(fb, cx, cy, sea_h as i32, app.sim.ticks);
    fb.draw_rect(
        -cx,
        -cy,
        WORLD[0] as u32,
        WORLD[1] as u32,
        Pixel::rgb(28, 95, 111),
    );
    for art in island_art() {
        fb.draw_image(art.origin[0] - cx, art.origin[1] - cy, &art.image);
    }
    ports(fb, cx, cy, sea_h as i32, app.sim.ticks);
    wake(fb, app, cx, cy);
    for island in ISLANDS {
        let x = island.rect[0] as i32 + 20 - cx;
        let y = island.rect[1] as i32 - 25 - cy;
        if (4..fb.width() as i32 - 150).contains(&x) && (10..sea_h as i32 - 15).contains(&y) {
            label(fb, x, y, island.name, MUTED);
        }
    }
    if app.paused {
        let x = fb.width() as i32 / 2 - 42;
        fb.fill_rect(x - 12, 18, 108, 29, INK);
        fb.draw_text_scaled(x, 26, "PAUSE", 2, GOLD);
    }
    let x = ((app.sim.position[0] + SCALE / 2) / SCALE) as i32 - cx;
    let y = ((app.sim.position[1] + SCALE / 2) / SCALE) as i32 - cy;
    // Draw after every world overlay; camera margins keep it above the HUD.
    ship(fb, x, y, app.sim.heading, app.sim.sails);
    hud(fb, app);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cached_terrain_and_town_footprints_match_solid_geometry() {
        for town in TOWNS {
            for (x, y, tower) in town {
                for yy in y - 12..=y + if tower { 30 } else { 18 } {
                    for xx in x - 2..=x + 20 {
                        assert!(
                            land_contains([i64::from(xx), i64::from(yy)]),
                            "building at {x},{y}, outside at {xx},{yy}"
                        );
                    }
                }
            }
        }
        for art in island_art() {
            for y in 0..art.image.height() {
                for x in 0..art.image.width() {
                    let p = [
                        i64::from(art.origin[0]) + i64::from(x),
                        i64::from(art.origin[1]) + i64::from(y),
                    ];
                    if !land_contains(p) {
                        let offset = ((y * art.image.width() + x) * 4) as usize;
                        let c = &art.image.as_rgba8()[offset..offset + 4];
                        assert!(
                            c[3] == 0
                                || [
                                    [156, 219, 205, 255],
                                    [35, 147, 143, 255],
                                    [22, 112, 126, 255],
                                    [16, 85, 109, 255]
                                ]
                                .iter()
                                .any(|water| c == water),
                            "solid decoration outside coastline: {p:?}"
                        );
                    }
                }
            }
        }
    }
}
