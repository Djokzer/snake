use crate::board;
use raylib::prelude::*;

pub struct Snake
{
	pub next  : (i32, i32),
	pub head  : (i32, i32),
	pub body  : Vec<(i32, i32)>,
	pub color : Color,
	pub eye_color : Color,
}

pub fn build_snake(_head : (i32, i32), board : &board::Board, screen_size : (i32, i32), _color : Color, _eye_color : Color) -> Snake
{
	Snake
	{
		head : _head,
		next : (_head.0, _head.1 + (screen_size.1 / board.x)),
		body : vec![],
		color : _color,
		eye_color : _eye_color,
	}
}

pub fn draw_snake(h : &mut RaylibDrawHandle, snake : &mut Snake, b : &board::Board, screen_size : (i32, i32))
{
	h.draw_rectangle(snake.head.0, snake.head.1, screen_size.0/b.x, screen_size.1/b.y, snake.color);
}

pub fn update_snake(snake : &mut Snake, block : (i32, i32), screen_size : (i32, i32))
{
	snake.head = snake.next;
    snake.next = (snake.next.0 % screen_size.0 ,(snake.next.1 + block.1) % screen_size.1);
}