use std::ops::{Add, AddAssign};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Collision {
	pub horizontal: bool,
	pub vertical: bool,
}

impl Collision {
	fn new(horizontal: bool, vertical: bool) -> Self {
		Self { horizontal, vertical }
	}
}

pub fn ball_collision(ball: Vector, ball_direction: Vector, paddle: &Paddle, bounds: Vector) -> Collision {
	let mut collision: Collision = Collision::new(false, false);
	let new_position = ball + ball_direction;

	if ball_direction == Vector::new(1,1) && new_position == *paddle.front().unwrap() {
		collision.horizontal = true;
		collision.vertical = true;
	} else if ball_direction == Vector::new(-1,1) && new_position == *paddle.back().unwrap() {
		collision.horizontal = true;
		collision.vertical = true;
	}

	if paddle.contains(&new_position) { collision.vertical = true; }
	else if new_position.y <= 0 { collision.vertical = true; }

	if new_position.x <= 0 { collision.horizontal = true; }
	else if new_position.x >= bounds.x - 1 { collision.horizontal = true; }

	collision
}

pub fn move_paddle(paddle: &mut Paddle, direction: Vector) {
	for segment in paddle.iter_mut() {
		*segment += direction;
	}
}

pub type Paddle = VecDeque<Vector>;
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector {
	pub x: i32,
	pub y: i32,
}

impl Vector {
	pub fn new(x: i32, y: i32) -> Self {
		Self { x,y }
	}
}

impl Add for Vector {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		Vector::new(self.x + other.x, self.y + other.y)
	}
}

impl AddAssign for Vector {
	fn add_assign(&mut self, other: Self) {
		*self = Self {
			x: self.x + other.x,
			y: self.y + other.y,
		};
	}
}
