mod frame_buffer;

use frame_buffer::FrameBuffer;
use minifb::{Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

fn main() {
    let mut position = Point::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0);
    let mut velocity = Point::new(1.0, 1.0);

    let mut frame_buffer = FrameBuffer::new(WIDTH, HEIGHT);

    let a = Point::new(1.0, 0.0);
    let b = Point::new(0.0, 1.0);
    let input = vec![a, a, a, a, b, b, b, b];
    let mut cycle = input.iter().cycle();

    let mut window = Window::new("Bounce", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    while window.is_open() {
        position += velocity;

        velocity = cycle.next().unwrap().clone();

        if position.y < 0.0 {
            position.y = (HEIGHT - 1) as f32;
        }

        if position.x < 0.0 {
            position.x = (WIDTH - 1) as f32;
        }

        if position.y >= (HEIGHT - 1) as f32 {
            position.y = 0.0;
        }

        if position.x >= (WIDTH - 1) as f32 {
            position.x = 0.0;
        }

        frame_buffer.toggle_pixel(position.x as usize, position.y as usize);

        window.update_with_buffer(&frame_buffer.buffer()).unwrap();
    }
}
