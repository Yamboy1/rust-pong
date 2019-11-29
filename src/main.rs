use pancurses::{curs_set, initscr, noecho, endwin, Window, Input};
use std::thread::sleep;
use std::time::Duration;
use std::collections::VecDeque;
use rust_pong::{Paddle, Vector, paddle_collision, move_paddle, CollisionPoint::*};

fn main() {
	let window = initscr();
	window.refresh();
	window.keypad(true);
	window.nodelay(true);
	curs_set(0);
	noecho();
	let (max_y,max_x) = window.get_max_yx();

	let board_bounds = Vector::new(max_x, max_y);
	let mut paddle: Paddle = VecDeque::new();
	for i in 0..10 {
		paddle.push_back(Vector::new(i, max_y-1));
	}
	let mut paddle_direction = Vector::new(0,0);
	let mut ball = Vector::new(3,3);
	let mut ball_direction = Vector::new(1,1);

	loop {
		let paddle_direction = get_new_paddle_direction(&window);
		move_paddle(&mut paddle, paddle_direction);
		ball = ball + ball_direction;
		display(&window, &paddle, ball);

		if ball.y > max_y { break; }

		ball_direction = match paddle_collision(&paddle, ball) {
			Some(collision) =>
				match collision {
					LeftCorner | RightCorner => Vector::new(-ball_direction.x, -ball_direction.y),
					Top => Vector::new(ball_direction.x, -ball_direction.y),
				}
			None => {
				let mut ball_direction = ball_direction;
				if ball.x == 0 || ball.x == max_x - 1 { ball_direction = Vector::new(-ball_direction.x, ball_direction.y); }
				if ball.y == 0 { ball_direction = Vector::new(ball_direction.x, -ball_direction.y); }
				ball_direction
			}
		};
		sleep(Duration::from_millis(100));
	}
	endwin();
}

fn display(window: &Window, paddle: &Paddle, ball: Vector) {
	window.clear();
	window.mvaddch(ball.y, ball.x, '@');
	for segment in paddle {
		window.mvaddch(segment.y, segment.x, '#');
	}
}

fn get_new_paddle_direction(window: &Window) -> Vector {
	match window.getch() {
		Some(Input::KeyLeft) => Vector::new(-2, 0),
		Some(Input::KeyRight) => Vector::new(2, 0),
		_ => Vector::new(0, 0),
	}
}