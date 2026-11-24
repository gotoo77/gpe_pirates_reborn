//! Deterministic P1 rules. Positions and camera origins use 1/1024 world units.
pub const SCALE: i64 = 1024;
pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 360;
pub const SEA_HEIGHT: u32 = 312;
pub const WORLD: [i64; 2] = [1600, 1000];
pub const HEADINGS: [[i64; 2]; 16] = [
    [1024, 0],
    [946, 392],
    [724, 724],
    [392, 946],
    [0, 1024],
    [-392, 946],
    [-724, 724],
    [-946, 392],
    [-1024, 0],
    [-946, -392],
    [-724, -724],
    [-392, -946],
    [0, -1024],
    [392, -946],
    [724, -724],
    [946, -392],
];
pub const HEADING_NAMES: [&str; 16] = [
    "E", "ESE", "SE", "SSE", "S", "SSO", "SO", "OSO", "O", "ONO", "NO", "NNO", "N", "NNE", "NE",
    "ENE",
];

#[derive(Clone, Copy, Debug)]
pub struct Island {
    pub name: &'static str,
    pub rect: [i64; 4],
}
pub const ISLANDS: [Island; 4] = [
    Island {
        name: "ILE DE LA BRISE",
        rect: [96, 160, 192, 240],
    },
    Island {
        name: "ILE DU PHARE",
        rect: [1120, 160, 224, 256],
    },
    Island {
        name: "ILE DE L'ANSE",
        rect: [608, 672, 256, 192],
    },
    Island {
        name: "GRANDE DORSALE",
        rect: [672, 192, 256, 320],
    },
];
#[derive(Clone, Copy, Debug)]
pub struct Port {
    pub name: &'static str,
    pub position: [i64; 2],
}
pub const PORTS: [Port; 3] = [
    Port {
        name: "PORT-BRISE",
        position: [320, 280],
    },
    Port {
        name: "PORT-PHARE",
        position: [1088, 288],
    },
    Port {
        name: "PORT-ANSE",
        position: [736, 640],
    },
];
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Wind {
    pub heading: u8,
    pub force: u8,
}
impl Default for Wind {
    fn default() -> Self {
        Self {
            heading: 0,
            force: 2,
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Command {
    pub rudder: i8,
    pub raise: bool,
    pub lower: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Simulation {
    pub position: [i64; 2],
    pub remainder: [i64; 2],
    pub heading: u8,
    pub rudder: i8,
    pub turn_ticks: u8,
    pub sails: bool,
    pub wind: Wind,
    pub ticks: u64,
    pub ports_inside: [bool; 3],
    pub last_port: usize,
    pub arrival: Option<usize>,
    pub blocked: bool,
    /// Hundredths of world units per second; zero for a rejected tick.
    pub effective_speed: u32,
}
impl Default for Simulation {
    fn default() -> Self {
        Self {
            position: [320 * SCALE, 280 * SCALE],
            remainder: [0, 0],
            heading: 0,
            rudder: 0,
            turn_ticks: 0,
            sails: false,
            wind: Wind::default(),
            ticks: 0,
            ports_inside: [true, false, false],
            last_port: 0,
            arrival: None,
            blocked: false,
            effective_speed: 0,
        }
    }
}
pub fn polar(heading: u8, wind: u8) -> u32 {
    let delta = (heading + 16 - wind) % 16;
    [85, 95, 100, 95, 80, 60, 40, 25, 15][usize::from(delta.min(16 - delta))]
}
pub fn speed(heading: u8, wind: Wind) -> u32 {
    [0, 48, 72, 96][usize::from(wind.force)] * polar(heading, wind.heading)
}

// Slab intersection with rational times: denominators are always positive.
// Coordinates stay within the 1600x1000 map, so i64 cross products suffice.
fn intersects(from: [i64; 2], to: [i64; 2], rect: [i64; 4]) -> bool {
    let mut enter = (0, 1);
    let mut leave = (1, 1);
    let greater = |a: (i64, i64), b: (i64, i64)| a.0 * b.1 > b.0 * a.1;
    for axis in 0..2 {
        let low = (rect[axis] - 6) * SCALE;
        let high = (rect[axis] + rect[axis + 2] + 6) * SCALE;
        let delta = to[axis] - from[axis];
        if delta == 0 {
            if from[axis] < low || from[axis] > high {
                return false;
            }
        } else {
            let (first, last) = if delta > 0 {
                ((low - from[axis], delta), (high - from[axis], delta))
            } else {
                ((from[axis] - high, -delta), (from[axis] - low, -delta))
            };
            if greater(first, enter) {
                enter = first;
            }
            if greater(leave, last) {
                leave = last;
            }
            if greater(enter, leave) {
                return false;
            }
        }
    }
    true
}

pub fn blocked_segment(from: [i64; 2], to: [i64; 2]) -> bool {
    for point in [from, to] {
        for axis in 0..2 {
            if point[axis] <= 6 * SCALE || point[axis] >= (WORLD[axis] - 6) * SCALE {
                return true;
            }
        }
    }
    ISLANDS
        .iter()
        .any(|island| intersects(from, to, island.rect))
}

pub fn camera(position: [i64; 2]) -> [i64; 2] {
    [
        (position[0] - i64::from(WIDTH / 2) * SCALE)
            .clamp(0, (WORLD[0] - i64::from(WIDTH)) * SCALE),
        (position[1] - i64::from(SEA_HEIGHT / 2) * SCALE)
            .clamp(0, (WORLD[1] - i64::from(SEA_HEIGHT)) * SCALE),
    ]
}

impl Simulation {
    pub fn tick(&mut self, command: Command) {
        if command.raise {
            self.sails = true;
            self.arrival = None;
        }
        if command.lower {
            self.sails = false;
        }
        let rudder = command.rudder.signum();
        if rudder == 0 {
            self.turn_ticks = 0;
        } else if rudder != self.rudder {
            self.turn_ticks = 0;
            self.heading = (i16::from(self.heading) + i16::from(rudder)).rem_euclid(16) as u8;
        } else {
            self.turn_ticks += 1;
            if self.turn_ticks == 10 {
                self.heading = (i16::from(self.heading) + i16::from(rudder)).rem_euclid(16) as u8;
                self.turn_ticks = 0;
            }
        }
        self.rudder = rudder;
        self.blocked = false;
        self.effective_speed = if self.sails {
            speed(self.heading, self.wind)
        } else {
            0
        };
        if self.effective_speed == 0 {
            self.remainder = [0, 0];
        } else {
            let mut proposed = self.position;
            for (axis, coordinate) in proposed.iter_mut().enumerate() {
                let numerator = HEADINGS[usize::from(self.heading)][axis]
                    * i64::from(self.effective_speed)
                    * 20
                    + self.remainder[axis];
                *coordinate += numerator / 100_000;
                self.remainder[axis] = numerator % 100_000;
            }
            if blocked_segment(self.position, proposed) {
                self.blocked = true;
                self.effective_speed = 0;
                self.remainder = [0, 0];
            } else {
                self.position = proposed;
            }
        }
        for (i, port) in PORTS.iter().enumerate() {
            let dx = self.position[0] - port.position[0] * SCALE;
            let dy = self.position[1] - port.position[1] * SCALE;
            let inside = dx * dx + dy * dy <= (24 * SCALE) * (24 * SCALE);
            if inside && !self.ports_inside[i] {
                self.arrival = Some(i);
                self.last_port = i;
                self.sails = false;
            }
            self.ports_inside[i] = inside;
        }
        self.ticks += 1;
    }
}
