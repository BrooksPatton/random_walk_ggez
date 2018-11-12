use rand::random;
use ggez::{Point2};
use ggez::graphics::{Color};
use super::Bullet;

pub struct RandomWalker {
    location: Point2,
    radius: f32,
    color: Color,
    pub bullet: Bullet,
    destination: Point2,
    velocity: Point2,
    speed: f32
}

impl RandomWalker {
    pub fn new(width: f32, height: f32) -> GameResult<RandomWalker> {
        let x = width / 2.0;
        let y = height / 2.0;
        let color = Color::new( random::<f32>(), 
                                random::<f32>(), 
                                random::<f32>(), 
                                1.0);
        let bullet = Bullet::new();
        let destination = Point2::new(x, y);
        let velocity = Point2::new(0f32, 0f32);
        let speed = 100f32;

        Ok(RandomWalker {
            location: Point2::new(x, y),
            radius: 15.0,
            color,
            bullet,
            destination,
            velocity,
            speed
        })
    }

    pub fn update(&mut self, game_width: f32, game_height: f32, dt: f32) {
        if !self.bullet.is_fired {
            let bullet_location = self.location.clone();
            let target = Point2::new(   random::<f32>() * game_width,
                                        random::<f32>() * game_height);
            self.bullet.fire(bullet_location, target);
        }

        if self.is_at_destination() {
            let x = random::<f32>() * game_width;
            let y = random::<f32>() * game_height;

            self.destination = Point2::new(x, y);
        }

        self.step(dt);
    }

    pub fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        set_color(context, self.color)?;
        circle(context, DrawMode::Line(1.0), self.location, self.radius, 1.0)
    }

    pub fn keep_in_arena(&mut self, arena_width: f32, arena_height: f32) -> GameResult<()> {
        if self.location.y < 0.0 {
            self.location.y = 0.0;
        } else if self.location.y > arena_height {
            self.location.y = arena_height;
        }

        if self.location.x < 0.0 {
            self.location.x = 0.0;
        } else if self.location.x > arena_width {
            self.location.x = arena_width;
        }

        Ok(())
    }

    pub fn is_at_destination(&self) -> bool {
        let difference = Point2::new(
            self.location.x - self.destination.x,
            self.location.y - self.destination.y
        );
        let distance = get_magnitude(difference);

        distance < 3f32
    }

    pub fn step(&mut self, dt: f32) {
        let direction = Point2::new(
            self.destination.x - self.location.x,
            self.destination.y - self.location.y
        );
        let mut normalized_direction = Point2::new(0f32, 0f32);

        if let Some(result) = normalize(direction) {
            normalized_direction = result;
        }
        
        let velocity = Point2::new(
            normalized_direction.x * self.speed,
            normalized_direction.y * self.speed
        );

        self.location.x += velocity.x * dt;
        self.location.y += velocity.y * dt;
    }
}

