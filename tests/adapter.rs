use gpe_pirates_reborn::{
    app::{App, Controls, select_gamepad, steering},
    navigation::Simulation,
};
use std::time::Duration;

fn ms(value: u64) -> Duration {
    Duration::from_millis(value)
}

#[test]
fn edge_waits_for_tick_and_five_ticks_do_not_repeat_raise() {
    let mut app = App::default();
    app.advance(
        ms(5),
        Controls {
            raise: true,
            ..Controls::default()
        },
    );
    assert_eq!(app.sim.ticks, 0);
    assert!(!app.sim.sails);
    app.advance(ms(15), Controls::default());
    assert_eq!(app.sim.ticks, 1);
    assert!(app.sim.sails);
    // Enter a port in the first of five ticks; repeating raise would restart it.
    app.sim = Simulation {
        position: [1088 * 1024, 313 * 1024],
        heading: 12,
        ports_inside: [false; 3],
        ..Simulation::default()
    };
    app.advance(
        ms(100),
        Controls {
            raise: true,
            ..Controls::default()
        },
    );
    assert_eq!(app.sim.ticks, 5);
    assert_eq!(app.sim.arrival, Some(1));
    assert!(!app.sim.sails);
}

#[test]
fn lower_wins_across_frames_pause_and_reset_clear_pending_time() {
    let mut a = App::default();
    a.advance(
        ms(5),
        Controls {
            raise: true,
            ..Controls::default()
        },
    );
    a.advance(
        ms(15),
        Controls {
            lower: true,
            ..Controls::default()
        },
    );
    assert!(!a.sim.sails);
    a.advance(
        ms(19),
        Controls {
            raise: true,
            ..Controls::default()
        },
    );
    a.advance(
        ms(100),
        Controls {
            pause: true,
            ..Controls::default()
        },
    );
    let snapshot = a.sim.clone();
    a.advance(
        ms(100),
        Controls {
            raise: true,
            ..Controls::default()
        },
    );
    assert_eq!(a.sim, snapshot);
    a.advance(
        ms(100),
        Controls {
            pause: true,
            ..Controls::default()
        },
    );
    a.advance(ms(1), Controls::default());
    assert_eq!(a.sim, snapshot);
    a.advance(ms(19), Controls::default());
    assert_eq!(a.sim.ticks, 2);
    assert!(!a.sim.sails);
    a.advance(
        ms(19),
        Controls {
            raise: true,
            ..Controls::default()
        },
    );
    a.advance(
        ms(100),
        Controls {
            reset: true,
            raise: true,
            pause: true,
            ..Controls::default()
        },
    );
    assert_eq!(a.sim, Simulation::default());
    assert!(!a.paused);
    a.advance(ms(1), Controls::default());
    assert_eq!(a.sim, Simulation::default());
}

#[test]
fn regrouping_callbacks_preserves_exact_state_for_tick_indexed_commands() {
    let mut a = App::default();
    let mut b = App::default();
    for group in 0..2000 {
        let c = Controls {
            rudder: if group % 2 == 0 { 1 } else { -1 },
            raise: group % 21 == 0,
            lower: group % 53 == 0,
            ..Controls::default()
        };
        a.advance(ms(100), c);
        b.advance(ms(3), c);
        b.advance(
            ms(17),
            Controls {
                raise: false,
                lower: false,
                ..c
            },
        );
        for _ in 0..4 {
            b.advance(
                ms(20),
                Controls {
                    raise: false,
                    lower: false,
                    ..c
                },
            );
        }
    }
    assert_eq!(a.sim.ticks, 10_000);
    assert_eq!(a.sim, b.sim);
}

#[test]
fn stable_controller_selection_and_direction_priorities() {
    assert_eq!(select_gamepad(None, &[9, 2, 5]), Some(2));
    assert_eq!(select_gamepad(Some(5), &[9, 2, 5]), Some(5));
    assert_eq!(select_gamepad(Some(5), &[9, 2]), Some(2));
    assert_eq!(select_gamepad(Some(2), &[]), None);
    assert_eq!(steering(true, true, false, true, 1.0), 0);
    assert_eq!(steering(true, false, false, true, 1.0), -1);
    assert_eq!(steering(false, false, true, true, 1.0), 0);
    assert_eq!(steering(false, false, true, false, 1.0), -1);
    assert_eq!(steering(false, false, false, false, 0.35), 1);
    assert_eq!(steering(false, false, false, false, -0.35), -1);
    assert_eq!(steering(false, false, false, false, 0.349), 0);
    assert_eq!(steering(false, false, false, false, 0.0), 0);
}
