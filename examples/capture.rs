//! Reproducible CPU framebuffer captures; PPM avoids a new dependency.
use gotoo_pixel_engine::{Framebuffer, Pixel};
use gpe_pirates_reborn::{
    app::{App, Controls},
    draw,
    navigation::{HEIGHT, SCALE, WIDTH},
};
use std::{
    fs,
    io::{self, Write},
    path::Path,
    time::Duration,
};

fn save(dir: &Path, name: &str, fb: &Framebuffer) -> io::Result<()> {
    let mut file = fs::File::create(dir.join(format!("{name}.ppm")))?;
    write!(file, "P6\n{} {}\n255\n", fb.width(), fb.height())?;
    let rgb: Vec<_> = fb
        .as_rgba8()
        .as_chunks::<4>()
        .0
        .iter()
        .flat_map(|p| p[..3].iter().copied())
        .collect();
    file.write_all(&rgb)
}
fn main() -> io::Result<()> {
    let output = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "target/captures".into());
    let dir = Path::new(&output);
    fs::create_dir_all(dir)?;
    let mut app = App::default();
    let mut fb = Framebuffer::new(WIDTH, HEIGHT);
    draw::scene(&mut fb, &app);
    save(dir, "p11-start-960", &fb)?;
    app.advance(
        Duration::from_millis(20),
        Controls {
            raise: true,
            ..Controls::default()
        },
    );
    for _ in 0..70 {
        app.advance(Duration::from_millis(20), Controls::default());
    }
    draw::scene(&mut fb, &app);
    save(dir, "p11-sailing-960", &fb)?;
    app.chart_open = true;
    draw::scene(&mut fb, &app);
    save(dir, "p11-chart-960", &fb)?;
    app.chart_open = false;
    let mut wide = Framebuffer::new(1280, 720);
    draw::scene(&mut wide, &app);
    save(dir, "p11-sailing-1280", &wide)?;
    app.sim.position = [1593 * SCALE, 993 * SCALE];
    app.sim.heading = 15;
    draw::scene(&mut fb, &app);
    save(dir, "p11-edge-960", &fb)?;
    let mut atlas = Framebuffer::new(512, 100);
    atlas.clear(Pixel::rgb(12, 66, 89));
    for heading in 0..16 {
        draw::ship(&mut atlas, 16 + i32::from(heading) * 32, 26, heading, true);
        draw::ship(&mut atlas, 16 + i32::from(heading) * 32, 74, heading, false);
    }
    save(dir, "p11-ship-16-headings", &atlas)?;
    println!("CPU captures: {}", dir.display());
    Ok(())
}
