use gotoo_pixel_engine::{
    EngineConfig, EngineError, Frame, Framebuffer, Game, GameResult, Image, ImageFilter, ImageFit,
    Input, Pixel, Rect,
    outline_text::OutlineFont,
    ui::{Ui, UiResponse, UiState, UiTheme, fonts::BuiltinOutlineFont},
};
use std::time::Duration;
const TEXT: &str = "Échange — quantité, coût, cargaison : 1 234 pièces";
const BACKGROUND: Pixel = Pixel::rgb(20, 38, 45);
const PAPER: Pixel = Pixel::rgb(227, 208, 170);
const TITLE: Rect = Rect {
    x: 32,
    y: 150,
    width: 896,
    height: 60,
};
const BODY: Rect = Rect {
    x: 32,
    y: 220,
    width: 896,
    height: 60,
};
pub struct Spike {
    state: UiState,
    font: OutlineFont,
    image: Image,
    pub count: u64,
    pub response: UiResponse,
}
impl Default for Spike {
    fn default() -> Self {
        Self::new()
    }
}
impl Spike {
    pub fn new() -> Self {
        Self {
            state: UiState::default(),
            font: BuiltinOutlineFont::Figtree.load().expect("bundled Figtree"),
            image: Image::decode_png(include_bytes!("../../assets/ui-spike/GPE.png"))
                .expect("licensed GPE PNG"),
            count: 0,
            response: UiResponse::default(),
        }
    }
    pub fn render(&mut self, fb: &mut Framebuffer, input: &Input, delta: Duration) {
        fb.clear(BACKGROUND);
        // Keep the toolkit's geometry, hit testing and activation together.
        let mut ui = Ui::new(
            fb,
            input,
            delta,
            &mut self.state,
            UiTheme {
                padding: 32,
                row_height: 48,
                row_spacing: 12,
                text: PAPER,
                ..UiTheme::default()
            },
        );
        self.response = ui.button("Activer");
        drop(ui);
        if self.response.clicked {
            self.count += 1;
        }
        self.font
            .draw(fb, "Pirates Reborn / S1", 34.0, TITLE, PAPER);
        self.font.draw(fb, TEXT, 24.0, BODY, PAPER);
        fb.draw_image_fit(
            &self.image,
            Rect {
                x: 32,
                y: 300,
                width: 160,
                height: 140,
            },
            ImageFit::Contain,
            ImageFilter::Linear,
        );
        self.font.draw(
            fb,
            &format!("Activations : {}", self.count),
            24.0,
            Rect {
                x: 32,
                y: 460,
                width: 896,
                height: 48,
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
            title: "Pirates Reborn — S1 capacity".into(),
            framebuffer_width: 960,
            framebuffer_height: 540,
            window_width: 960,
            window_height: 540,
        },
        Spike::new(),
    )
}
#[cfg(not(target_arch = "wasm32"))]
pub fn capture_and_measure(directory: &std::path::Path) -> std::io::Result<()> {
    use std::{hint::black_box, io::Write, time::Instant};
    std::fs::create_dir_all(directory)?;
    let start = Instant::now();
    let mut spike = Spike::new();
    println!("load_ms={:.3}", start.elapsed().as_secs_f64() * 1000.0);
    let mut fb = Framebuffer::new(960, 540);
    let input = Input::default();
    let start = Instant::now();
    spike.render(&mut fb, &input, Duration::ZERO);
    println!(
        "first_draw_ms={:.3}",
        start.elapsed().as_secs_f64() * 1000.0
    );
    println!(
        "glyphs={} body_size={:?} png={}x{}",
        spike.font.supports_text(TEXT),
        spike.font.measure(TEXT, 24.0, BODY),
        spike.image.width(),
        spike.image.height()
    );
    let mut file = std::fs::File::create(directory.join("s1-cpu.ppm"))?;
    write!(file, "P6\n960 540\n255\n")?;
    for pixel in fb.as_rgba8().as_chunks::<4>().0 {
        file.write_all(&pixel[..3])?;
    }
    let mut samples = Vec::new();
    for _ in 0..1000 {
        let start = Instant::now();
        spike.render(black_box(&mut fb), &input, Duration::from_millis(16));
        samples.push(start.elapsed().as_secs_f64() * 1000.0);
    }
    samples.sort_by(f64::total_cmp);
    println!(
        "warm_draw_ms median={:.3} p95={:.3} samples=1000",
        samples[500], samples[950]
    );
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn licensed_resources_load_and_french_text_fits() {
        let mut s = Spike::new();
        assert!(s.font.supports_text(TEXT));
        let size = s.font.measure(TEXT, 24.0, BODY);
        assert!(size.width > 0 && size.width <= BODY.width && size.height <= BODY.height);
        assert!(s.image.width() > 0 && s.image.height() > 0);
    }
    #[test]
    fn idle_frames_do_not_activate() {
        let mut s = Spike::new();
        let mut fb = Framebuffer::new(960, 540);
        for _ in 0..3 {
            s.render(&mut fb, &Input::default(), Duration::ZERO);
        }
        assert_eq!(s.count, 0);
        assert!(s.response.focused);
        assert!(!s.response.hovered);
    }
}
