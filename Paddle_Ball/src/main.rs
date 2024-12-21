use ggez::{
    event::{self, EventHandler},
    graphics::{self, Canvas, Color, DrawParam},
    glam::Vec2,
    Context, GameResult,
};

struct PaddleBallGame {
    ball_pos: Vec2,
    ball_vel: Vec2,
    screen_size: Vec2,
}

impl PaddleBallGame {
    fn new(screen_width: f32, screen_height: f32) -> Self {
        PaddleBallGame {
            ball_pos: Vec2::new(screen_width / 2.0, screen_height / 2.0),
            ball_vel: Vec2::new(300.0, 250.0), // Pixels per second
            screen_size: Vec2::new(screen_width, screen_height),
        }
    }
}

impl EventHandler for PaddleBallGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32();

        // Update ball position
        self.ball_pos += self.ball_vel * dt;

        // Check for collision with screen edges
        if self.ball_pos.x <= 0.0 || self.ball_pos.x >= self.screen_size.x {
            self.ball_vel.x = -self.ball_vel.x; // Reverse horizontal velocity
        }
        if self.ball_pos.y <= 0.0 || self.ball_pos.y >= self.screen_size.y {
            self.ball_vel.y = -self.ball_vel.y; // Reverse vertical velocity
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        // Draw the ball
        let ball = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.ball_pos,
            20.0, // Ball radius
            0.1,
            Color::WHITE,
        )?;
        canvas.draw(&ball, DrawParam::default());

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ggez::ContextBuilder::new("Paddle Ball", "Your Name")
        .build()
        .expect("Failed to create ggez context");

    let screen_width = 1000.0;
    let screen_height = 750.0;
    let game = PaddleBallGame::new(screen_width, screen_height);

    event::run(ctx, event_loop, game)
}
