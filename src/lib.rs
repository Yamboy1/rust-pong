use std::ops::Add;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum CollisionPoint {
	LeftCorner,
	RightCorner,
	Top
}

pub fn paddle_collision(paddle: &Paddle, object: CoordinateVector) -> Option<CollisionPoint> {
	if object + CoordinateVector(1,1) == *paddle.front().unwrap() {
		Some(CollisionPoint::LeftCorner)
	} else if object + CoordinateVector(-1,1) == *paddle.back().unwrap() {
		Some(CollisionPoint::RightCorner)
	} else if paddle.contains(&(object + CoordinateVector(0,1))) {
		Some(CollisionPoint::Top)
	} else {
		None
	}
}

pub fn move_paddle(paddle: &mut Paddle, direction: CoordinateVector) {
	for segment in paddle.iter_mut() {
		segment.0 += direction.0;
	}
}

pub type Paddle = VecDeque<CoordinateVector>;
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CoordinateVector(pub i32,pub i32);
impl Add for CoordinateVector {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		CoordinateVector(self.0 + rhs.0, self.1 + rhs.1)
	}
}
