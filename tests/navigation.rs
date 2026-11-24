use gpe_pirates_reborn::navigation::{
    Command, HEADINGS, ISLANDS, PORTS, SCALE, Simulation, Wind, blocked_segment, camera, polar,
    speed,
};

fn sail(rudder: i8) -> Command {
    Command {
        rudder,
        raise: true,
        lower: false,
    }
}

#[test]
fn polar_all_forces_directions_and_wrap_are_exact() {
    let percentages = [
        85, 95, 100, 95, 80, 60, 40, 25, 15, 25, 40, 60, 80, 95, 100, 95,
    ];
    for w in 0..16 {
        for (offset, percentage) in percentages.into_iter().enumerate() {
            let h = (w + offset as u8) % 16;
            assert_eq!(polar(h, w), percentage);
            for (force, base) in [0, 48, 72, 96].into_iter().enumerate() {
                assert_eq!(
                    speed(
                        h,
                        Wind {
                            heading: w,
                            force: force as u8
                        }
                    ),
                    base * percentage
                );
            }
        }
    }
    assert_eq!(speed(2, Wind::default()), 7200);
    assert_eq!(speed(4, Wind::default()), 5760);
    assert_eq!(speed(8, Wind::default()), 1080);
}

#[test]
fn movement_retains_signed_remainders_and_stops_exactly() {
    for (heading, expected) in [
        (0, [62_668, 0]),
        (2, [52_128, 52_128]),
        (8, [-11_059, 0]),
        (10, [-20_851, -20_851]),
    ] {
        let mut s = Simulation {
            position: [480 * SCALE, 560 * SCALE],
            heading,
            ..Simulation::default()
        };
        let initial = s.position;
        for _ in 0..50 {
            s.tick(sail(0));
        }
        assert_eq!(
            [s.position[0] - initial[0], s.position[1] - initial[1]],
            expected,
            "heading {heading}"
        );
        assert!(s.remainder.iter().all(|r| r.abs() < 100_000));
        s.tick(Command {
            lower: true,
            ..Command::default()
        });
        assert_eq!(s.remainder, [0, 0]);
        assert_eq!(s.effective_speed, 0);
        let stopped = s.position;
        for _ in 0..50 {
            s.tick(Command::default());
        }
        assert_eq!(s.position, stopped);
    }
    let mut s = Simulation {
        wind: Wind {
            heading: 0,
            force: 0,
        },
        ..Simulation::default()
    };
    s.tick(sail(0));
    assert_eq!(s.position, [320 * SCALE, 280 * SCALE]);
    assert_eq!(s.remainder, [0, 0]);
}

#[test]
fn rudder_immediate_repeat_release_reverse_and_wrap() {
    let mut s = Simulation::default();
    s.tick(Command {
        rudder: -1,
        ..Command::default()
    });
    assert_eq!(s.heading, 15);
    for _ in 0..9 {
        s.tick(Command {
            rudder: -1,
            ..Command::default()
        });
    }
    assert_eq!(s.heading, 15);
    s.tick(Command {
        rudder: -1,
        ..Command::default()
    });
    assert_eq!(s.heading, 14);
    s.tick(Command {
        rudder: 1,
        ..Command::default()
    });
    assert_eq!(s.heading, 15);
    s.tick(Command::default());
    s.tick(Command {
        rudder: 1,
        ..Command::default()
    });
    assert_eq!(s.heading, 0);
    assert_eq!(s.position, [320 * SCALE, 280 * SCALE]);
}

#[test]
fn swept_collision_catches_crossings_angles_tangency_and_world_edges() {
    let p = |x, y| [x * SCALE, y * SCALE];
    for (a, b) in [
        ((320, 280), (80, 280)),
        ((500, 300), (1000, 300)),
        ((500, 186), (1000, 186)),
        ((650, 170), (700, 220)),
        ((500, 100), (6, 100)),
        ((500, 100), (1594, 100)),
        ((500, 100), (500, 6)),
        ((500, 900), (500, 994)),
    ] {
        assert!(blocked_segment(p(a.0, a.1), p(b.0, b.1)), "{a:?}->{b:?}");
    }
    for (a, b) in [
        ((500, 185), (1000, 185)),
        ((480, 560), (1024, 560)),
        ((7, 7), (1593, 7)),
        ((500, 7), (500, 993)),
    ] {
        assert!(!blocked_segment(p(a.0, a.1), p(b.0, b.1)), "{a:?}->{b:?}");
    }
    // One sub-unit clearance is legal; touching an expanded corner is not.
    assert!(!blocked_segment(
        [500 * SCALE, 186 * SCALE - 1],
        [1000 * SCALE, 186 * SCALE - 1]
    ));
    assert!(blocked_segment(p(666, 186), p(666, 186)));
}

#[test]
fn collision_rejects_whole_tick_and_turning_allows_escape() {
    let mut s = Simulation {
        position: [665 * SCALE, 300 * SCALE],
        ..Simulation::default()
    };
    s.tick(sail(0));
    assert!(s.blocked);
    assert_eq!(s.position, [665 * SCALE, 300 * SCALE]);
    assert_eq!(s.remainder, [0, 0]);
    assert_eq!(s.effective_speed, 0);
    for _ in 0..8 {
        s.tick(Command {
            rudder: 1,
            ..Command::default()
        });
        s.tick(Command::default());
    }
    assert!(s.position[0] < 665 * SCALE);
    assert!(!s.blocked);
}

#[test]
fn ports_are_water_arrival_only_on_entry_and_can_leave_and_return() {
    assert_eq!(ISLANDS.len(), 4);
    for port in PORTS {
        for offset in [[-24, 0], [24, 0], [0, -24], [0, 24], [0, 0]] {
            let pos = [
                (port.position[0] + offset[0]) * SCALE,
                (port.position[1] + offset[1]) * SCALE,
            ];
            assert!(!blocked_segment(pos, pos));
        }
    }
    let mut s = Simulation::default();
    s.tick(sail(0));
    assert!(s.sails);
    assert_eq!(s.arrival, None);
    for _ in 0..24 {
        s.tick(Command::default());
    }
    assert!(!s.ports_inside[0]);
    // Turn in place, then sail back into the arrival disk.
    s.tick(Command {
        lower: true,
        ..Command::default()
    });
    for _ in 0..8 {
        s.tick(Command {
            rudder: 1,
            ..Command::default()
        });
        s.tick(Command::default());
    }
    for _ in 0..100 {
        s.tick(sail(0));
        if s.arrival == Some(0) {
            break;
        }
    }
    assert_eq!(s.arrival, Some(0));
    assert!(!s.sails);
    assert_eq!(s.last_port, 0);
    s.tick(sail(0));
    assert!(s.sails, "inside disk must not stop the ship again");
    s = Simulation::default();
    assert_eq!(s, Simulation::default());
}

#[test]
fn camera_tracks_subunits_and_clamps_all_edges() {
    assert_eq!(camera([320 * SCALE, 280 * SCALE]), [0, 124 * SCALE]);
    assert_eq!(
        camera([800 * SCALE + 17, 500 * SCALE + 23]),
        [480 * SCALE + 17, 344 * SCALE + 23]
    );
    assert_eq!(camera([7 * SCALE, 7 * SCALE]), [0, 0]);
    assert_eq!(
        camera([1593 * SCALE, 993 * SCALE]),
        [960 * SCALE, 688 * SCALE]
    );
}

#[test]
fn deterministic_full_state_for_ten_thousand_ticks() {
    let mut a = Simulation::default();
    let mut b = a.clone();
    for tick in 0..10_000 {
        let c = Command {
            rudder: if tick % 127 < 60 { 1 } else { -1 },
            raise: tick % 200 == 0,
            lower: tick % 391 == 390,
        };
        a.tick(c);
        b.tick(c);
        assert_eq!(a, b);
        assert!(!blocked_segment(a.position, a.position));
    }
    assert_eq!(a.ticks, 10_000);
    assert!(
        HEADINGS
            .iter()
            .all(|[x, y]| (x * x + y * y - 1_048_576).abs() < 2048)
    );
}

#[test]
fn recorded_crossing_and_routes_to_anse_and_back_are_navigable() {
    let mut s = Simulation::default();
    for line in include_str!("fixtures/brise-phare.txt")
        .lines()
        .filter(|line| !line.starts_with('#'))
    {
        let fields: Vec<i32> = line
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        for repeat in 0..fields[0] {
            let before = s.position;
            s.tick(Command {
                rudder: fields[1] as i8,
                raise: repeat == 0 && fields[2] == 1,
                lower: repeat == 0 && fields[3] == 1,
            });
            assert!(!s.blocked);
            assert!(!blocked_segment(before, s.position));
        }
    }
    assert_eq!(s.arrival, Some(1));
    assert_eq!(s.last_port, 1);
    assert!(!s.sails);
    assert!(s.ticks < 9000);
    fn turn(s: &mut Simulation, heading: u8) {
        s.tick(Command {
            lower: true,
            ..Command::default()
        });
        while s.heading != heading {
            s.tick(Command {
                rudder: 1,
                ..Command::default()
            });
            s.tick(Command::default());
        }
    }
    fn travel(s: &mut Simulation, heading: u8, axis: usize, threshold: i64, increasing: bool) {
        turn(s, heading);
        s.tick(sail(0));
        for _ in 0..5000 {
            if (increasing && s.position[axis] >= threshold * SCALE)
                || (!increasing && s.position[axis] <= threshold * SCALE)
            {
                return;
            }
            assert!(s.sails, "unexpected arrival {:?}", s.arrival);
            s.tick(Command::default());
            assert!(!s.blocked);
        }
        panic!("route never reached its waypoint");
    }
    travel(&mut s, 4, 1, 560, true);
    travel(&mut s, 8, 0, 736, false);
    turn(&mut s, 4);
    s.tick(sail(0));
    for _ in 0..100 {
        s.tick(Command::default());
        assert!(!s.blocked);
        if s.arrival == Some(2) {
            break;
        }
    }
    assert_eq!(s.arrival, Some(2));
    assert!(!s.sails);
    travel(&mut s, 12, 1, 560, false);
    travel(&mut s, 8, 0, 320, false);
    turn(&mut s, 12);
    s.tick(sail(0));
    for _ in 0..300 {
        s.tick(Command::default());
        assert!(!s.blocked);
        if s.arrival == Some(0) {
            break;
        }
    }
    assert_eq!(s.arrival, Some(0));
    assert!(!s.sails);
}
