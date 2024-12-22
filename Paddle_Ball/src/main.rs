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
    ball_radius: f32,
    left_paddle: LeftPaddle,
    right_paddle: RightPaddle,
}

struct LeftPaddle {
    size_x: f32,
    size_y: f32,
    position: Vec2,
    velocity_y: f32, // Only need vertical velocity (positive or negative)
}

struct RightPaddle {
    size_x: f32,
    size_y: f32,
    position: Vec2,
    velocity_y: f32, // Only need vertical velocity (positive or negative)
}

impl PaddleBallGame {
    fn new(screen_width: f32, screen_height: f32) -> Self {
        PaddleBallGame {
            ball_pos: Vec2::new(screen_width / 2.0, screen_height / 2.0),
            ball_vel: Vec2::new(650.0, 400.0), // Ball speed in pixels per second
            screen_size: Vec2::new(screen_width, screen_height),
            ball_radius: 18.0, // Ball radius
            left_paddle: LeftPaddle {
                size_x: 10.0,
                size_y: 150.0,
                position: Vec2::new(10.0, screen_height / 2.0 - 50.0), // Position of the paddle on the left
                velocity_y: 0.0, // Paddle starts stationary
            },
            right_paddle: RightPaddle {
                size_x: 10.0,
                size_y: 150.0,
                position: Vec2::new(1180.0, screen_height / 2.0 - 50.0), // Position of the paddle on the right
                velocity_y: 0.0, // Paddle starts stationary
            },
        }
    }
}

impl EventHandler for PaddleBallGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32();

        // Update ball position
        self.ball_pos += self.ball_vel * dt;

        // Check for collision with screen edges, considering ball radius
        if self.ball_pos.x - self.ball_radius <= 0.0 || self.ball_pos.x + self.ball_radius >= self.screen_size.x {
            self.ball_vel.x = -self.ball_vel.x; // Reverse horizontal velocity
        }
        if self.ball_pos.y - self.ball_radius <= 0.0 || self.ball_pos.y + self.ball_radius >= self.screen_size.y {
            self.ball_vel.y = -self.ball_vel.y; // Reverse vertical velocity
        }

        // Update left paddle position based on velocity
        self.left_paddle.position.y += self.left_paddle.velocity_y * dt;

        // Keep paddle within screen bounds
        if self.left_paddle.position.y < 0.0 {
            self.left_paddle.position.y = 0.0;
        }
        if self.left_paddle.position.y + self.left_paddle.size_y > self.screen_size.y {
            self.left_paddle.position.y = self.screen_size.y - self.left_paddle.size_y;
        }

        // Update right paddle position based on velocity
        self.right_paddle.position.y += self.right_paddle.velocity_y * dt;

        // Keep paddle within screen bounds
        if self.right_paddle.position.y < 0.0 {
            self.right_paddle.position.y = 0.0;
        }
        if self.right_paddle.position.y + self.right_paddle.size_y > self.screen_size.y {
            self.right_paddle.position.y = self.screen_size.y - self.right_paddle.size_y;
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
            self.ball_radius,
            0.1,
            Color::WHITE,
        )?;
        canvas.draw(&ball, DrawParam::default());

        // Draw the left paddle as a rectangle
        let left_paddle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(self.left_paddle.position.x, self.left_paddle.position.y, self.left_paddle.size_x, self.left_paddle.size_y),
            Color::WHITE,
        )?;
        canvas.draw(&left_paddle, DrawParam::default());

        // Draw the right paddle as a rectangle
        let right_paddle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(self.right_paddle.position.x, self.right_paddle.position.y, self.right_paddle.size_x, self.right_paddle.size_y),
            Color::WHITE,
        )?;
        canvas.draw(&right_paddle, DrawParam::default());


        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("Paddle Ball", "Your Name")
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()
        .expect("Failed to create ggez context");

    let screen_width = 1200.0;
    let screen_height = 900.0;

    let game = PaddleBallGame::new(screen_width, screen_height);

    event::run(ctx, event_loop, game)
}
