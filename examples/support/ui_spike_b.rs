use std::time::Duration;

use gotoo_pixel_engine::{
    ActionId, ControlMap, EngineConfig, EngineError, Frame, Framebuffer, Game, GameResult,
    GamepadButton, Image, ImageFilter, ImageFit, Input, Key, MouseButton, Pixel, Rect,
    outline_text::OutlineFont,
    ui::{
        fonts::BuiltinOutlineFont,
        resolved::{
            self, ResolvedOutput, ResolvedState, ResolvedTarget, UiId, UiInput, UiNavInput,
            UiPointerInput,
        },
        standard_menu_controls,
    },
};

const UP: ActionId = ActionId::new("up");
const DOWN: ActionId = ActionId::new("down");
const LEFT: ActionId = ActionId::new("left");
const RIGHT: ActionId = ActionId::new("right");
const CONFIRM: ActionId = ActionId::new("confirm");
const CANCEL: ActionId = ActionId::new("cancel");
const SUSPEND: ActionId = ActionId::new("suspend");
const TOGGLE: ActionId = ActionId::new("toggle");
const PAPER: Pixel = Pixel::rgb(227, 208, 170);
const LABELS: [&str; 5] = ["Centre", "Haut", "Droite", "Bas", "Gauche"];

pub struct Spike {
    state: ResolvedState,
    controls: ControlMap,
    targets: [ResolvedTarget; 5],
    flash: Option<(UiId, Duration)>,
    font: OutlineFont,
    image: Image,
    count: u64,
}

impl Default for Spike {
    fn default() -> Self {
        Self::new()
    }
}

impl Spike {
    pub fn new() -> Self {
        let mut controls = standard_menu_controls(UP, DOWN, CONFIRM);
        controls.bind_key(CONFIRM, Key::Enter);
        for (action, key, button) in [
            (LEFT, Key::Left, GamepadButton::DPadLeft),
            (RIGHT, Key::Right, GamepadButton::DPadRight),
            (CANCEL, Key::Escape, GamepadButton::East),
            (SUSPEND, Key::R, GamepadButton::Start),
            (TOGGLE, Key::D, GamepadButton::West),
        ] {
            controls.bind_key(action, key).bind_gamepad(action, button);
        }
        // Irregular geometry chosen by this scene; these exact rects are painted.
        let rects = [
            Rect {
                x: 330,
                y: 220,
                width: 200,
                height: 70,
            },
            Rect {
                x: 360,
                y: 130,
                width: 150,
                height: 50,
            },
            Rect {
                x: 660,
                y: 210,
                width: 250,
                height: 80,
            },
            Rect {
                x: 300,
                y: 370,
                width: 260,
                height: 70,
            },
            Rect {
                x: 50,
                y: 230,
                width: 160,
                height: 60,
            },
        ];
        Self {
            state: ResolvedState::default(),
            controls,
            targets: std::array::from_fn(|index| ResolvedTarget {
                id: UiId::from_stable(index as u64 + 1).unwrap(),
                rect: rects[index],
                action: ActionId::new(["center", "up", "right", "down", "left"][index]),
                enabled: index != 3,
            }),
            flash: None,
            font: BuiltinOutlineFont::Figtree.load().expect("bundled Figtree"),
            image: Image::decode_png(include_bytes!("../../assets/ui-spike/GPE.png"))
                .expect("licensed GPE PNG"),
            count: 0,
        }
    }

    fn interact(&mut self, input: UiInput<'_>, delta: Duration) -> ResolvedOutput {
        let output = resolved::update(&mut self.state, input, &self.targets)
            .expect("fixed disjoint visible rectangles and stable IDs");
        if let Some((_, time)) = &mut self.flash {
            *time = time.saturating_sub(delta);
        }
        // A single dispatch point; navigation and paint never dispatch actions.
        if let Some(activation) = output.activation {
            self.count += 1;
            self.flash = Some((activation.id, Duration::from_millis(180)));
        }
        if output.cancelled {
            self.flash = None;
        }
        output
    }

    pub fn render(&mut self, fb: &mut Framebuffer, input: &Input, delta: Duration) {
        // Every OS snapshot reaches the engine barrier, including unfocused ones.
        let ready = self.state.synchronize_input(input);
        if !ready {
            self.flash = None;
        }
        self.controls.update(input);
        if ready && self.controls.action(SUSPEND).pressed() {
            self.state.suspend();
            self.flash = None;
        }
        if ready && self.controls.action(TOGGLE).pressed() {
            self.targets[3].enabled = !self.targets[3].enabled;
        }
        let output = self.interact(
            UiInput {
                nav: UiNavInput {
                    up: self.controls.action(UP).pressed(),
                    down: self.controls.action(DOWN).pressed(),
                    left: self.controls.action(LEFT).pressed(),
                    right: self.controls.action(RIGHT).pressed(),
                    confirm: self.controls.action(CONFIRM).pressed(),
                    cancel: self.controls.action(CANCEL).pressed(),
                },
                pointer: UiPointerInput {
                    position: input.mouse_position(),
                    pressed: input.mouse_button(MouseButton::Left).pressed(),
                    released: input.mouse_button(MouseButton::Left).released(),
                },
                touches: input.touches(),
            },
            delta,
        );
        self.paint(fb, &output);
    }

    fn paint(&mut self, fb: &mut Framebuffer, output: &ResolvedOutput) {
        fb.clear(Pixel::rgb(20, 38, 45));
        self.font.draw(
            fb,
            "Pirates Reborn / S1-B",
            34.0,
            Rect {
                x: 32,
                y: 20,
                width: 750,
                height: 50,
            },
            PAPER,
        );
        self.font.draw(
            fb,
            "Échange — quantité, coût, cargaison",
            24.0,
            Rect {
                x: 32,
                y: 76,
                width: 750,
                height: 40,
            },
            PAPER,
        );
        fb.draw_image_fit(
            &self.image,
            Rect {
                x: 820,
                y: 20,
                width: 100,
                height: 90,
            },
            ImageFit::Contain,
            ImageFilter::Linear,
        );
        for (index, target) in self.targets.iter().enumerate() {
            let fill = if !target.enabled {
                Pixel::rgb(58, 58, 58)
            } else if output.active(target.id) {
                Pixel::rgb(50, 145, 120)
            } else if output.hovered == Some(target.id) {
                Pixel::rgb(45, 75, 110)
            } else {
                Pixel::rgb(30, 43, 60)
            };
            let rect = target.rect;
            fb.fill_rect(rect.x, rect.y, rect.width, rect.height, fill);
            let flash = self
                .flash
                .is_some_and(|(id, time)| id == target.id && !time.is_zero());
            let border = if flash {
                Pixel::rgb(255, 180, 70)
            } else if output.focused == Some(target.id) {
                Pixel::rgb(120, 235, 255)
            } else {
                Pixel::rgb(80, 85, 95)
            };
            fb.draw_rect(rect.x, rect.y, rect.width, rect.height, border);
            self.font.draw(
                fb,
                if target.enabled {
                    LABELS[index]
                } else {
                    "Indisponible"
                },
                22.0,
                Rect {
                    x: rect.x + 8,
                    y: rect.y + 8,
                    width: rect.width - 16,
                    height: rect.height - 16,
                },
                PAPER,
            );
        }
        self.font.draw(
            fb,
            &format!("Activations : {}", self.count),
            24.0,
            Rect {
                x: 32,
                y: 478,
                width: 896,
                height: 40,
            },
            PAPER,
        );
    }
}

impl Game for Spike {
    fn update(&mut self, frame: &mut Frame<'_>) -> GameResult {
        self.render(frame.framebuffer, frame.input, frame.delta_time);
        GameResult::Continue
    }
}

pub fn start() -> Result<(), EngineError> {
    gotoo_pixel_engine::run(
        EngineConfig {
            title: "Pirates Reborn — S1-B resolved".into(),
            framebuffer_width: 960,
            framebuffer_height: 540,
            window_width: 960,
            window_height: 540,
        },
        Spike::new(),
    )
}

#[cfg(not(target_arch = "wasm32"))]
pub fn capture(directory: &std::path::Path) -> std::io::Result<()> {
    use std::io::Write;
    std::fs::create_dir_all(directory)?;
    let mut spike = Spike::new();
    let mut fb = Framebuffer::new(960, 540);
    spike.render(&mut fb, &Input::default(), Duration::ZERO);
    let mut file = std::fs::File::create(directory.join("s1-b-cpu.ppm"))?;
    write!(file, "P6\n960 540\n255\n")?;
    for pixel in fb.as_rgba8().as_chunks::<4>().0 {
        file.write_all(&pixel[..3])?;
    }
    Ok(())
}

#[cfg(test)]
#[path = "ui_spike_b_tests.rs"]
mod tests;
