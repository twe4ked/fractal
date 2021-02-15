mod frame_buffer;

use frame_buffer::FrameBuffer;
use minifb::{Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

#[derive(Clone, Copy, PartialEq, Debug)]
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

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn point(&self) -> Point {
        match self {
            Direction::Up => Point::new(0.0, -1.0),
            Direction::Down => Point::new(0.0, 1.0),
            Direction::Left => Point::new(-1.0, 0.0),
            Direction::Right => Point::new(1.0, 0.0),
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
}

fn main() {
    let mut position = Point::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0);
    let mut direction = Direction::Left;

    let mut frame_buffer = FrameBuffer::new(WIDTH, HEIGHT);

    let mut i = 0;
    let mut j = 0;

    let mut window = Window::new("Bounce", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    while window.is_open() {
        // std::thread::sleep(std::time::Duration::from_millis(1));

        if i % 4 == 0 {
            direction = if turn_left_or_right(j) {
                direction.turn_left()
            } else {
                direction.turn_right()
            };

            j += 1;
        }

        position += direction.point();

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

        i += 1;
    }
}

fn turn_left_or_right(n: usize) -> bool {
    let n = n as isize;
    (((n & -n) << 1) & n) != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_test() {
        let up = Direction::Up;
        let down = Direction::Down;
        let left = Direction::Left;
        let right = Direction::Right;

        // Turn left
        assert_eq!(up.turn_left(), left);
        assert_eq!(down.turn_left(), right);
        assert_eq!(left.turn_left(), down);
        assert_eq!(right.turn_left(), up);

        // Turn right
        assert_eq!(up.turn_right(), right);
        assert_eq!(down.turn_right(), left);
        assert_eq!(left.turn_right(), up);
        assert_eq!(right.turn_right(), down);
    }
}
