pub mod app;
pub mod draw;
pub mod economy;
pub mod navigation;
use gotoo_pixel_engine::{EngineConfig, EngineError, Frame, Game, GameResult, run};
use navigation::{HEIGHT, WIDTH};

impl Game for app::App {
    fn update(&mut self, frame: &mut Frame<'_>) -> GameResult {
        let controls = self.read_controls(frame.input);
        self.advance(frame.delta_time, controls);
        draw::scene(frame.framebuffer, self);
        GameResult::Continue
    }
}

pub fn start() -> Result<(), EngineError> {
    run(
        EngineConfig {
            title: "Pirates Reborn - P2-X1".into(),
            framebuffer_width: WIDTH,
            framebuffer_height: HEIGHT,
            window_width: WIDTH * 3 / 2,
            window_height: HEIGHT * 3 / 2,
        },
        app::App::default(),
    )
}
