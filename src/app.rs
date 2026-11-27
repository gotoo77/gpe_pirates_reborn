//! Frame input and fixed-step adapter; no renderer is needed to test it.
use crate::{
    economy::{CAPACITY, Economy, GOODS, Quote, Side, TradeError},
    navigation::{Command, Simulation},
};
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
    pub market: bool,
    pub market_close: bool,
    pub market_side: bool,
    pub market_row: i8,
    pub market_quantity: i8,
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
    pub economy: Economy,
    pub market_port: Option<usize>,
    pub market_good: usize,
    pub market_side: Side,
    pub market_quantity: u8,
    pub market_feedback: String,
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
    pub fn docked_port(&self) -> Option<usize> {
        if self.sim.sails {
            return None;
        }
        self.sim.ports_inside.iter().position(|inside| *inside)
    }

    pub fn market_quote(&self) -> Result<Quote, TradeError> {
        let port = self.market_port.ok_or(TradeError::Invalid)?;
        if self.docked_port() != Some(port) {
            return Err(TradeError::Invalid);
        }
        self.economy.quote(
            port,
            self.market_good,
            self.market_side,
            self.market_quantity,
        )
    }

    fn close_market(&mut self) {
        self.market_port = None;
        self.navigation_latched = true;
        self.clear_pending();
    }

    fn advance_market(&mut self, controls: Controls) {
        if self.market_port != self.docked_port() || controls.pause || controls.market_close {
            self.close_market();
            return;
        }
        if controls.chart {
            return;
        }
        if controls.market_row != 0 || controls.market_quantity != 0 || controls.market_side {
            self.market_good = (self.market_good as i8 + controls.market_row.signum())
                .rem_euclid(GOODS.len() as i8) as usize;
            self.market_quantity = (i16::from(self.market_quantity)
                + i16::from(controls.market_quantity.signum()))
            .clamp(1, i16::from(CAPACITY)) as u8;
            if controls.market_side {
                self.market_side = match self.market_side {
                    Side::Buy => Side::Sell,
                    Side::Sell => Side::Buy,
                };
            }
            self.market_feedback.clear();
            // A changed selection must be drawn before any confirmation.
            return;
        }
        if controls.market {
            self.market_feedback = match self.market_quote() {
                Ok(quote) => match self.economy.execute(quote) {
                    Ok(()) => format!(
                        "{} {} {} : {} OR",
                        if self.market_side == Side::Buy {
                            "ACHAT"
                        } else {
                            "VENTE"
                        },
                        self.market_quantity,
                        GOODS[self.market_good],
                        quote.total()
                    ),
                    Err(error) => error.message().to_owned(),
                },
                Err(error) => error.message().to_owned(),
            };
        }
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
            market: pressed(Key::Enter, GamepadButton::South),
            market_close: self.gamepad.is_some_and(|id| {
                input.gamepad_button(GamepadId::new(id), GamepadButton::East).pressed()
            }),
            market_side: pressed(Key::Tab, GamepadButton::North),
            market_row: i8::from(pressed(Key::Down, GamepadButton::DPadDown))
                - i8::from(pressed(Key::Up, GamepadButton::DPadUp)),
            market_quantity: i8::from(pressed(Key::Right, GamepadButton::DPadRight))
                - i8::from(pressed(Key::Left, GamepadButton::DPadLeft)),
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
            self.economy = Economy::default();
            self.market_port = None;
            self.market_good = 0;
            self.market_side = Side::Buy;
            self.market_quantity = 0;
            self.market_feedback.clear();
            self.paused = false;
            self.chart_open = false;
            self.navigation_latched = false;
            self.wake.clear();
            self.clear_pending();
            return;
        }
        if self.market_port.is_some() {
            self.advance_market(controls);
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
        if controls.market && !controls.market_close
            && let Some(port) = self.docked_port()
        {
            self.economy.observe(port);
            self.market_port = Some(port);
            self.market_good = 0;
            self.market_side = Side::Buy;
            self.market_quantity = 1;
            self.market_feedback.clear();
            self.clear_pending();
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
            let before_ports = self.sim.ports_inside;
            self.sim.tick(Command {
                rudder: controls.rudder,
                raise: std::mem::take(&mut self.pending_raise),
                lower: std::mem::take(&mut self.pending_lower),
            });
            let after_ports = self.sim.ports_inside;
            if !before_ports.iter().any(|inside| *inside)
                || !after_ports.iter().any(|inside| *inside)
            {
                self.economy.sea_tick();
            }
            for (port, (&before, &after)) in before_ports.iter().zip(&after_ports).enumerate() {
                if !before && after {
                    self.economy.observe(port);
                }
            }
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
