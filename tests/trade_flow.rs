use gpe_pirates_reborn::{
    app::{App, Controls},
    economy::{Economy, Side, START_CASH, TICKS_PER_DAY},
    navigation::{SCALE, Simulation},
};
use std::time::Duration;

fn tick(app: &mut App, controls: Controls) {
    app.advance(Duration::from_millis(20), controls);
}

fn open(app: &mut App) {
    tick(
        app,
        Controls {
            market: true,
            ..Controls::default()
        },
    );
    assert!(app.market_port.is_some());
}

fn close(app: &mut App) {
    tick(
        app,
        Controls {
            market_close: true,
            ..Controls::default()
        },
    );
    assert!(app.market_port.is_none());
    tick(app, Controls::default()); // Neutrality re-arms navigation.
}

#[test]
fn market_confirmation_is_contextual_and_single_edge() {
    let mut app = App::default();
    let before = app.economy.clone();
    open(&mut app);
    assert_eq!(app.economy, before);
    assert_eq!(app.sim.ticks, 0);
    let quote = app.market_quote().unwrap();
    tick(
        &mut app,
        Controls {
            market: true,
            ..Controls::default()
        },
    );
    assert_eq!(app.economy.cash(), START_CASH - quote.total());
    assert_eq!(app.economy.cargo(), [1, 0, 0]);
    let before = app.economy.clone();
    app.advance(Duration::from_secs(5), Controls::default());
    assert_eq!(app.economy, before);
    assert_eq!(app.sim.ticks, 0);
    tick(
        &mut app,
        Controls {
            market: true,
            market_quantity: 1,
            ..Controls::default()
        },
    );
    assert_eq!(app.economy, before);
    assert_eq!(app.market_quantity, 2);
    tick(
        &mut app,
        Controls {
            market: true,
            market_close: true,
            raise: true,
            ..Controls::default()
        },
    );
    assert_eq!(app.economy, before);
    assert!(!app.sim.sails);
    assert!(app.market_port.is_none());
    tick(
        &mut app,
        Controls {
            raise: true,
            nautical_held: true,
            ..Controls::default()
        },
    );
    assert!(!app.sim.sails);
}

#[test]
fn sea_clock_freezes_in_port_and_menus_and_reset_clears_all() {
    let mut app = App::default();
    for _ in 0..TICKS_PER_DAY {
        tick(&mut app, Controls::default());
    }
    assert_eq!(app.economy.day(), 0);
    app.sim = Simulation {
        position: [400 * SCALE, 280 * SCALE],
        ports_inside: [false; 3],
        ..Simulation::default()
    };
    tick(
        &mut app,
        Controls {
            market: true,
            ..Controls::default()
        },
    );
    assert!(app.market_port.is_none()); // last_port alone is insufficient.
    for _ in 1..199 {
        tick(&mut app, Controls::default());
    }
    assert_eq!(app.economy.sea_ticks(), 199);
    let before = app.economy.clone();
    tick(
        &mut app,
        Controls {
            chart: true,
            ..Controls::default()
        },
    );
    app.advance(Duration::from_secs(10), Controls::default());
    assert_eq!(app.economy, before);
    tick(
        &mut app,
        Controls {
            chart: true,
            ..Controls::default()
        },
    );
    tick(&mut app, Controls::default());
    assert_eq!(app.economy.day(), 1);
    assert_eq!(app.economy.sea_ticks(), 0);
    tick(
        &mut app,
        Controls {
            pause: true,
            ..Controls::default()
        },
    );
    app.advance(Duration::from_secs(10), Controls::default());
    assert_eq!(app.economy.day(), 1);
    tick(
        &mut app,
        Controls {
            reset: true,
            ..Controls::default()
        },
    );
    assert_eq!(app.sim, Simulation::default());
    assert_eq!(app.economy, Economy::default());
}

fn trade(app: &mut App, good: usize, side: Side, quantity: u8) {
    open(app);
    app.market_good = good;
    app.market_side = side;
    app.market_quantity = quantity;
    let total = app.market_quote().unwrap().total();
    let cash = app.economy.cash();
    tick(
        app,
        Controls {
            market: true,
            ..Controls::default()
        },
    );
    assert_eq!(
        app.economy.cash(),
        match side {
            Side::Buy => cash - total,
            Side::Sell => cash + total,
        }
    );
    close(app);
}

fn turn(app: &mut App, heading: u8) {
    tick(
        app,
        Controls {
            lower: true,
            ..Controls::default()
        },
    );
    for _ in 0..16 {
        if app.sim.heading == heading {
            return;
        }
        tick(
            app,
            Controls {
                rudder: 1,
                ..Controls::default()
            },
        );
        tick(app, Controls::default());
    }
    assert_eq!(app.sim.heading, heading);
}

fn raise(app: &mut App) {
    tick(
        app,
        Controls {
            raise: true,
            ..Controls::default()
        },
    );
}

fn travel(app: &mut App, heading: u8, axis: usize, threshold: i64, increasing: bool) {
    turn(app, heading);
    raise(app);
    for _ in 0..5000 {
        let coordinate = app.sim.position[axis];
        if (increasing && coordinate >= threshold * SCALE)
            || (!increasing && coordinate <= threshold * SCALE)
        {
            return;
        }
        assert!(app.sim.sails);
        tick(app, Controls::default());
        assert!(!app.sim.blocked);
    }
    panic!("waypoint not reached");
}

#[test]
fn three_port_trade_replays_existing_navigation_route() {
    let mut app = App::default();
    trade(&mut app, 0, Side::Buy, 8);
    assert_eq!(app.economy.cargo(), [8, 0, 0]);
    let brise = app.economy.observation(0).unwrap();
    for line in include_str!("fixtures/brise-phare.txt")
        .lines()
        .filter(|line| !line.starts_with('#'))
    {
        let fields: Vec<i32> = line
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        for repeat in 0..fields[0] {
            tick(
                &mut app,
                Controls {
                    rudder: fields[1] as i8,
                    raise: repeat == 0 && fields[2] == 1,
                    lower: repeat == 0 && fields[3] == 1,
                    ..Controls::default()
                },
            );
            assert!(!app.sim.blocked);
        }
    }
    assert_eq!(app.docked_port(), Some(1));
    assert!(app.economy.day() > 0);
    assert_eq!(app.economy.observation(0), Some(brise));
    assert_eq!(
        app.economy.observation(1).unwrap().day,
        app.economy.day()
    );
    trade(&mut app, 0, Side::Sell, 8);
    assert!(app.economy.cash() > START_CASH);
    trade(&mut app, 1, Side::Buy, 5);
    travel(&mut app, 4, 1, 560, true);
    travel(&mut app, 8, 0, 736, false);
    turn(&mut app, 4);
    raise(&mut app);
    for _ in 0..100 {
        tick(&mut app, Controls::default());
        assert!(!app.sim.blocked);
        if app.docked_port() == Some(2) {
            break;
        }
    }
    assert_eq!(app.docked_port(), Some(2));
    assert_eq!(app.economy.cargo(), [0, 5, 0]);
    trade(&mut app, 1, Side::Sell, 5);
    assert_eq!(app.economy.used(), 0);
    assert!(app.economy.cash() > START_CASH);
    assert!(app.economy.observation(2).is_some());
}
