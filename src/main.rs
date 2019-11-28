use pancurses::{curs_set, initscr, noecho, endwin, Window, Input};
use std::thread::sleep;
use std::time::Duration;
use std::collections::VecDeque;
use rust_pong::{Paddle, CoordinateVector, paddle_collision, move_paddle, CollisionPoint::*};

fn main() {
	let window = initscr();
	window.refresh();
	window.keypad(true);
	window.nodelay(true);
	curs_set(0);
	noecho();
	let (max_y,max_x) = window.get_max_yx();

	let mut paddle: Paddle = VecDeque::new();
	for i in 0..10 {
		paddle.push_back(CoordinateVector(i, max_y-1));
	}
	let mut paddle_direction = CoordinateVector(0,0);
	let mut ball = CoordinateVector(3,3);
	let mut ball_direction = CoordinateVector(1,1);

	loop {
		let paddle_direction = get_new_paddle_direction(&window);
		move_paddle(&mut paddle, paddle_direction);
		ball = ball + ball_direction;
		display(&window, &paddle, ball);

		if ball.1 > max_y { break; }

		ball_direction = match paddle_collision(&paddle, ball) {
			Some(collision) =>
				match collision {
					LeftCorner | RightCorner => CoordinateVector(-ball_direction.0, -ball_direction.1),
					Top => CoordinateVector(ball_direction.0, -ball_direction.1),
				}
			None => {
				let mut ball_direction = ball_direction;
				if ball.0 == 0 || ball.0 == max_x - 1 { ball_direction = CoordinateVector(-ball_direction.0, ball_direction.1); }
				if ball.1 == 0 {
					ball_direction = CoordinateVector(ball_direction.0, -ball_direction.1);
				}
				ball_direction
			}
		};
		sleep(Duration::from_millis(100));
	}
	endwin();
}

fn display(window: &Window, paddle: &Paddle, ball: CoordinateVector) {
	window.clear();
	window.mvaddch(ball.1, ball.0, '@');
	for segment in paddle {
		window.mvaddch(segment.1, segment.0, '#');
	}
}

fn get_new_paddle_direction(window: &Window) -> CoordinateVector {
	match window.getch() {
		Some(Input::KeyLeft) => CoordinateVector(-2, 0),
		Some(Input::KeyRight) => CoordinateVector(2, 0),
		_ => CoordinateVector(0, 0),
	}
}