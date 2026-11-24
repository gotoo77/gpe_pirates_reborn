//! Frame input and fixed-step adapter; no renderer is needed to test it.
use crate::navigation::{Command, Simulation};
use gotoo_pixel_engine::{GamepadAxis, GamepadButton, GamepadId, Input, Key};
use std::collections::VecDeque;
use std::time::Duration;

#[derive(Clone, Copy, Debug, Default)]
pub struct Controls {
    pub rudder: i8,
    pub raise: bool,
    pub lower: bool,
    pub pause: bool,
    pub reset: bool,
    pub chart: bool,
    /// Raw held nautical inputs, before opposite keys or input priorities cancel.
    pub nautical_held: bool,
}

pub fn select_gamepad(current: Option<usize>, connected: &[usize]) -> Option<usize> {
    current
        .filter(|id| connected.contains(id))
        .or_else(|| connected.iter().copied().min())
}

pub fn steering(left: bool, right: bool, pad_left: bool, pad_right: bool, axis: f32) -> i8 {
    if left || right {
        i8::from(right) - i8::from(left)
    } else if pad_left || pad_right {
        i8::from(pad_right) - i8::from(pad_left)
    } else if axis >= 0.35 {
        1
    } else if axis <= -0.35 {
        -1
    } else {
        0
    }
}

#[derive(Default)]
pub struct App {
    pub sim: Simulation,
    pub paused: bool,
    pub chart_open: bool,
    /// Presentation only: accepted positions, never used by navigation.
    pub wake: VecDeque<([i64; 2], u64, u8)>,
    navigation_latched: bool,
    accumulator: Duration,
    pending_raise: bool,
    pending_lower: bool,
    gamepad: Option<usize>,
}

impl App {
    pub fn awaiting_neutral(&self) -> bool {
        self.navigation_latched
    }
    pub fn read_controls(&mut self, input: &Input) -> Controls {
        let connected: Vec<_> = input.gamepad_ids().map(GamepadId::as_usize).collect();
        self.gamepad = select_gamepad(self.gamepad, &connected);
        let pressed = |key, button| {
            input.key(key).pressed()
                || self
                    .gamepad
                    .is_some_and(|id| input.gamepad_button(GamepadId::new(id), button).pressed())
        };
        let held = |button| {
            self.gamepad
                .is_some_and(|id| input.gamepad_button(GamepadId::new(id), button).held())
        };
        let axis = self.gamepad.map_or(0.0, |id| {
            input.gamepad_axis(GamepadId::new(id), GamepadAxis::LeftStickX)
        });
        Controls {
            rudder: steering(
                input.key(Key::Left).held(),
                input.key(Key::Right).held(),
                held(GamepadButton::DPadLeft),
                held(GamepadButton::DPadRight),
                axis,
            ),
            raise: pressed(Key::Up, GamepadButton::DPadUp),
            lower: pressed(Key::Down, GamepadButton::DPadDown),
            pause: pressed(Key::Escape, GamepadButton::Start),
            reset: pressed(Key::R, GamepadButton::Select),
            chart: pressed(Key::M, GamepadButton::West),
            nautical_held: [Key::Left, Key::Right, Key::Up, Key::Down]
                .iter()
                .any(|&key| input.key(key).held())
                || [
                    GamepadButton::DPadLeft,
                    GamepadButton::DPadRight,
                    GamepadButton::DPadUp,
                    GamepadButton::DPadDown,
                ]
                .iter()
                .any(|&button| held(button))
                || axis.abs() >= 0.35,
        }
    }

    pub fn advance(&mut self, delta: Duration, mut controls: Controls) {
        if controls.reset {
            self.sim = Simulation::default();
            self.paused = false;
            self.chart_open = false;
            self.navigation_latched = false;
            self.wake.clear();
            self.clear_pending();
            return;
        }
        if controls.chart || (self.chart_open && controls.pause) {
            self.chart_open = !self.chart_open;
            self.navigation_latched = true;
            self.clear_pending();
            return;
        }
        if self.chart_open {
            return;
        }
        if controls.pause {
            self.paused = !self.paused;
            self.clear_pending();
            return;
        }
        if self.paused {
            return;
        }
        if self.navigation_latched {
            if !controls.nautical_held && controls.rudder == 0 && !controls.raise && !controls.lower
            {
                self.navigation_latched = false;
            }
            controls.rudder = 0;
            controls.raise = false;
            controls.lower = false;
        }
        self.pending_raise |= controls.raise;
        self.pending_lower |= controls.lower;
        self.accumulator += delta.min(Duration::from_millis(100));
        let step = Duration::from_millis(20);
        while self.accumulator >= step {
            self.accumulator -= step;
            let previous = self.sim.position;
            self.sim.tick(Command {
                rudder: controls.rudder,
                raise: std::mem::take(&mut self.pending_raise),
                lower: std::mem::take(&mut self.pending_lower),
            });
            if self.sim.position != previous && self.sim.ticks.is_multiple_of(3) {
                self.wake
                    .push_back((previous, self.sim.ticks, self.sim.heading));
            }
            while self
                .wake
                .front()
                .is_some_and(|(_, tick, _)| self.sim.ticks - tick > 90)
            {
                self.wake.pop_front();
            }
        }
    }

    fn clear_pending(&mut self) {
        self.accumulator = Duration::ZERO;
        self.pending_raise = false;
        self.pending_lower = false;
    }
}
