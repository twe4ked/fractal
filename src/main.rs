mod frame_buffer;

use frame_buffer::FrameBuffer;
use minifb::{Window, WindowOptions};

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

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

fn main() {
    let mut position = Point::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0);
    let mut velocity = Point::new(0.0, 0.0);

    let mut frame_buffer = FrameBuffer::new(WIDTH, HEIGHT);

    // let input = vec![a, a, a, a, b, b, b, b];
    // let mut cycle = input.iter().cycle();
    let mut i = 0;
    let mut j = 0;

    let mut window = Window::new("Bounce", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    while window.is_open() {
        std::thread::sleep(std::time::Duration::from_millis(1));

        if i % 10 == 0 {
            velocity = if turn(j) {
                turn_left(velocity)
            } else {
                turn_right(velocity)
            };
            j += 1;
        }

        position += velocity;

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

fn turn_left(point: Point) -> Point {
    if point.x == 0.0 {
        Point::new(if point.y > 0.0 { 1.0 } else { -1.0 }, 0.0)
    } else if point.y == 0.0 {
        Point::new(0.0, if point.x > 0.0 { -1.0 } else { 1.0 })
    } else {
        unreachable!()
    }
}

fn turn_right(point: Point) -> Point {
    if point.x == 0.0 {
        Point::new(if point.y > 0.0 { -1.0 } else { 1.0 }, 0.0)
    } else if point.y == 0.0 {
        Point::new(0.0, if point.x > 0.0 { 1.0 } else { -1.0 })
    } else {
        unreachable!()
    }
}

// fn turn_inner(point: Point, a: f32, b: f32) -> Point {
//     if point.x == 0.0 {
//         Point::new(if point.y > 0.0 { -1.0 } else { 1.0 }, 0.0)
//     } else if point.y == 0.0 {
//         Point::new(0.0, if point.x > 0.0 { 1.0 } else { -1.0 })
//     } else {
//         unreachable!()
//     }
// }

fn turn(n: usize) -> bool {
    let n = n as isize;
    (((n & -n) << 1) & n) != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_left_test() {
        let up = Point::new(0.0, -1.0);
        let down = Point::new(0.0, 1.0);
        let left = Point::new(-1.0, 0.0);
        let right = Point::new(1.0, 0.0);

        assert_eq!(turn_left(up), left);
        assert_eq!(turn_left(down), right);
        assert_eq!(turn_left(left), down);
        assert_eq!(turn_left(right), up);
    }

    #[test]
    fn turn_right_test() {
        let up = Point::new(0.0, -1.0);
        let down = Point::new(0.0, 1.0);
        let left = Point::new(-1.0, 0.0);
        let right = Point::new(1.0, 0.0);

        assert_eq!(turn_right(up), right);
        assert_eq!(turn_right(down), left);
        assert_eq!(turn_right(left), up);
        assert_eq!(turn_right(right), down);
    }
}
