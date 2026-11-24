use gpe_pirates_reborn::navigation::{Command, Simulation};

pub fn reference_state() -> String {
    let mut sim = Simulation::default();
    for tick in 0..10_000 {
        if tick == 5000 {
            sim = Simulation::default();
        }
        sim.tick(Command {
            rudder: if tick % 127 < 60 { 1 } else { -1 },
            raise: tick % 200 == 0,
            lower: tick % 391 == 390,
        });
    }
    format!("{sim:?}")
}
