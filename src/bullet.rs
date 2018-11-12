struct Bullet {
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