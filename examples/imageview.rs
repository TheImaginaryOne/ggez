extern crate cgmath;
extern crate ggez;
extern crate rand;

use ggez::audio;
use ggez::event;
use ggez::filesystem;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::timer;
use ggez::{Context, GameResult};
use std::env;
use std::path;

struct MainState {
    a: i32,
    direction: i32,
    image: graphics::Image,
    text: graphics::Text,
    pixel_sized_text: graphics::Text,
    // Not actually dead, see BUGGO below
    #[allow(dead_code)]
    sound: audio::Source,
}

impl MainState {
    fn draw_crazy_lines(&self, ctx: &mut Context) -> GameResult {
        let num_lines = 100;
        let mut colors = Vec::new();
        for _ in 0..num_lines {
            let r: u8 = rand::random();
            let g: u8 = rand::random();
            let b: u8 = rand::random();
            colors.push(Color::from((r, g, b, 255)));
        }

        let mut last_point = cgmath::Point2::new(400.0, 300.0);
        for color in colors {
            let x = (rand::random::<i32>() % 50) as f32;
            let y = (rand::random::<i32>() % 50) as f32;
            let point = cgmath::Point2::new(last_point.x + x, last_point.y + y);
            graphics::line(ctx, color, &[last_point, point], 3.0)?;
            last_point = point;
        }

        Ok(())
    }

    fn new(ctx: &mut Context) -> GameResult<MainState> {
        filesystem::print_all(ctx);

        let image = graphics::Image::new(ctx, "/dragon1.png").unwrap();

        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf").unwrap();
        let text = graphics::Text::new(("Hello world!", font, 48.0));
        let sound = audio::Source::new(ctx, "/sound.ogg").unwrap();

        let pixel_sized_text = graphics::Text::new(("This text is 32 pixels high", font, 32.0));

        let _ = sound.play();

        let s = MainState {
            a: 0,
            direction: 1,
            image,
            text,
            pixel_sized_text,
            // BUGGO: We never use sound again,
            // but we have to hang on to it, Or Else!
            // The optimizer will decide we don't need it
            // since play() has "no side effects" and free it.
            sound,
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.a += self.direction;
            if self.a > 250 || self.a <= 0 {
                self.direction *= -1;

                println!("Delta frame time: {:?} ", timer::get_delta(ctx));
                println!("Average FPS: {}", timer::get_fps(ctx));
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let c = self.a as u8;
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let color = Color::from((c, c, c, 255));
        let dest_point = cgmath::Point2::new(0.0, 0.0);
        graphics::draw(ctx, &self.image, (dest_point, 0.0, color))?;
        graphics::draw(ctx, &self.text, (dest_point, 0.0, color))?;

        let dest_point2 = cgmath::Point2::new(0.0, 256.0);
        graphics::rectangle(
            ctx,
            Color::from((0, 0, 0, 255)),
            graphics::DrawMode::Fill,
            graphics::Rect::new(0.0, 256.0, 500.0, 32.0),
        )?;
        graphics::draw(
            ctx,
            &self.pixel_sized_text,
            (dest_point2, 0.0, graphics::WHITE),
        )?;

        self.draw_crazy_lines(ctx)?;
        graphics::present(ctx)?;

        timer::yield_now();
        Ok(())
    }
}

// Creating a gamestate depends on having an SDL context to load resources.
// Creating a context depends on loading a config file.
// Loading a config file depends on having FS (or we can just fake our way around it
// by creating an FS and then throwing it away; the costs are not huge.)
pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("imageview", "ggez")
        .add_resource_path(resource_dir)
        ;
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
