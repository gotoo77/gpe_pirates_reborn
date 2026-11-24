use gpe_pirates_reborn::navigation::{ISLANDS, SCALE, blocked_segment, land_contains};

#[test]
fn organic_coasts_stay_in_p1_envelopes_and_square_sweep_covers_all_land() {
    let mut carved_water = 0;
    for island in ISLANDS {
        for part in island.parts {
            for i in 0..part.len() {
                let a = part[i];
                let b = part[(i + 1) % part.len()];
                let c = part[(i + 2) % part.len()];
                assert!(
                    (b[0] - a[0]) * (c[1] - b[1]) - (b[1] - a[1]) * (c[0] - b[0]) >= 0,
                    "convex lobes required"
                );
            }
        }
        let [x, y, w, h] = island.rect;
        for yy in (y - 8..=y + h + 8).step_by(7) {
            for xx in (x - 8..=x + w + 8).step_by(7) {
                let p = [xx * SCALE, yy * SCALE];
                if land_contains([xx, yy]) {
                    assert!(xx >= x && xx <= x + w && yy >= y && yy <= y + h);
                    assert!(blocked_segment(p, p));
                } else if xx > x && xx < x + w && yy > y && yy < y + h {
                    carved_water += 1;
                }
                if !blocked_segment(p, p) {
                    for oy in -6..=6 {
                        for ox in -6..=6 {
                            assert!(!land_contains([xx + ox, yy + oy]));
                        }
                    }
                }
            }
        }
    }
    assert!(
        carved_water > 200,
        "corners and bays must replace rectangles"
    );
}

#[test]
fn bevelled_coast_sweep_does_not_tunnel_or_use_old_bounding_rectangle() {
    let p = |x, y| [x * SCALE, y * SCALE];
    assert!(
        !blocked_segment(p(105, 165), p(110, 165)),
        "old corner is sea"
    );
    assert!(blocked_segment(p(70, 280), p(330, 280)));
    assert!(blocked_segment(p(1000, 288), p(1400, 288)));
}
