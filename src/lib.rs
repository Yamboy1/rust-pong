use std::ops::{Add, AddAssign};
use std::collections::VecDeque;

#[derive(Debug)]
pub enum CollisionPoint {
	LeftCorner,
	RightCorner,
	Top
}

pub fn paddle_collision(paddle: &Paddle, object: Vector) -> Option<CollisionPoint> {
	if object + Vector::new(1, 1) == *paddle.front().unwrap() {
		Some(CollisionPoint::LeftCorner)
	} else if object + Vector::new(-1, 1) == *paddle.back().unwrap() {
		Some(CollisionPoint::RightCorner)
	} else if paddle.contains(&(object + Vector::new(0, 1))) {
		Some(CollisionPoint::Top)
	} else {
		None
	}
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
