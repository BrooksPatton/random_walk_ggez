extern crate ggez;
extern crate rand;

mod random_walker;

use ggez::event::EventHandler;
use ggez::{GameResult, Context, event, graphics, timer};
use ggez::conf::Conf;
use ggez::graphics::{circle, DrawMode, present, Point2, Color, set_color};
use random_walker::RandomWalker;

use std::fs::File;

const TARGET_FPS: u32 = 60;

struct GameState {
    walkers: Vec<RandomWalker>,
    width: f32,
    height: f32
}

impl GameState {
    fn new(width: f32, height: f32) -> GameResult<GameState> {
        let mut walkers = Vec::new();

        walkers.push(RandomWalker::new(width, height)?);

        Ok(GameState {
            walkers,
            width,
            height
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        while timer::check_update_time(context, TARGET_FPS) {
            let dt = 1.0 / TARGET_FPS as f32;

            for walker in &mut self.walkers {
                walker.update(self.width, self.height, dt);
                walker.keep_in_arena(self.width, self.height)?;
                walker.bullet.update(context, dt, self.width, self.height)?;
            }
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context);

        for walker in &mut self.walkers {
            walker.draw(context)?;
            walker.bullet.draw(context)?;
        }

        present(context);
        Ok(())
    }
}

pub struct Bullet {
    location: Point2,
    velocity: Point2,
    size: f32,
    is_fired: bool,
    color: Color
}

impl Bullet {
    fn new() -> Bullet {
        let size = 5.0;
        let velocity = Point2::new(500.0, 0.0);
        let is_fired = false;
        let color = Color::new(1.0, 1.0, 1.0, 1.0);
        let location = Point2::new(-5.0, -5.0);

        Bullet {
            location,
            velocity,
            size,
            is_fired,
            color
        }
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        set_color(context, self.color)?;
        circle(context, DrawMode::Fill, self.location, self.size, 1.0)?;

        Ok(())
    }

    fn update(&mut self, _context: &mut Context, dt: f32, width: f32, height: f32) -> GameResult<()> {
        if self.is_fired {
            self.location.x += self.velocity.x * dt;
            self.location.y += self.velocity.y * dt;

            if self.is_off_screen(width, height) {
                self.is_fired = false;
            }
        }

        Ok(())
    }

    fn is_off_screen(&self, width: f32, height: f32) -> bool {
        self.location.y < 0.0 || self.location.x > width || self.location.y > height || self.location.x < 0.0
    }

    fn fire(&mut self, location: Point2, target: Point2) {
        let mut direction = Point2::new(target.x - location.x, target.y - location.y);

        if let Some(point) = normalize(direction) {
            direction = point;
        }

        direction.x *= 500f32;
        direction.y *= 500f32;

        self.velocity = direction;
        self.location = location;
        self.is_fired = true;
    }
}

fn get_magnitude(vector: Point2) -> f32 {
    let magnitude_squared = (vector.x * vector.x) + (vector.y * vector.y);

    magnitude_squared.sqrt()
}

fn normalize(vector: Point2) -> Option<Point2> {
    let magnitude = get_magnitude(vector);

    if magnitude > 0.0 {
        Some(Point2::new(
            vector.x / magnitude,
            vector.y / magnitude
        ))
    } else {
        None
    }
}

fn main() {
    let mut configuration_read = File::open("conf.toml").unwrap();
    let configuration = Conf::from_toml_file(&mut configuration_read).unwrap();
    let context = &mut Context::load_from_conf("random_walkers", "Brookzerker", configuration).unwrap();
    let (width, height) = graphics::get_size(context);
    let game_state = &mut GameState::new(width as f32, height as f32).unwrap();

    event::run(context, game_state).unwrap();
}
