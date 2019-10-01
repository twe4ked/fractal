mod frame_buffer;

use frame_buffer::FrameBuffer;
use minifb::{Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

fn main() {
    let mut position = Point::new(0.0, 0.0);
    let mut velocity = Point::new(1.0, 1.0);

    let mut frame_buffer = FrameBuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new("Bounce", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    while window.is_open() {
        position.x += velocity.x;
        position.y += velocity.y;

        if position.y < 0.0 {
            position.y = 0.0;
            velocity.y = -velocity.y;
        }

        if position.x < 0.0 {
            position.x = 0.0;
            velocity.x = -velocity.x;
        }

        if position.y >= (HEIGHT - 1) as f32 {
            position.y = (HEIGHT - 1) as f32;
            velocity.y = -velocity.y;
        }

        if position.x >= (WIDTH - 1) as f32 {
            position.x = (WIDTH - 1) as f32;
            velocity.x = -velocity.x;
        }

        frame_buffer.toggle_pixel(position.x as usize, position.y as usize);

        window.update_with_buffer(&frame_buffer.buffer()).unwrap();
    }
}
