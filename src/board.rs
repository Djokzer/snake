use raylib::prelude::*;
use Iterator;
use crate::snake;

pub struct Board{
	pub x : i32,
	pub y : i32,
	pub color_1 : Color,
	pub color_2 : Color,
	pub fruit : (i32, i32),
}

pub fn build_board(x_ : i32, y_ : i32, color_1_ : Color, color_2_ : Color, screen_size : (i32, i32)) -> Board
{
	Board
	{
		x : x_,
		y : y_,
		color_1 : color_1_,
		color_2 : color_2_,
		fruit : ((x_- 1) * (screen_size.0 / x_), 0),
	}
}

pub fn draw_board(h : &mut RaylibDrawHandle, b : &Board, screen_size : (i32, i32), block : (i32, i32))
{
	let mut c : Color;
	for x in (0..screen_size.0).step_by(block.0 as usize)
	{
		for y in (0..screen_size.1).step_by(block.1 as usize)
		{
			if ((x/block.0) + (y/block.1)) % 2 == 0{c = b.color_1;}
			else{c = b.color_2;}
			h.draw_rectangle(x, y, block.0, block.1, c);
		}
	}
	
	h.draw_rectangle(b.fruit.0, b.fruit.1, block.0 , block.1, Color::PURPLE);
}

pub fn update_board(b : &mut Board, snake : &mut snake::Snake, screen_size : (i32, i32), block : (i32, i32))
{
	if snake.body[0] == b.fruit
	{
		let r_1 : i32 = get_random_value(0, screen_size.0 / block.0 - 1);
		let r_2 : i32 = get_random_value(0, screen_size.1 / block.1  -1);
		b.fruit = (r_1 * block.0 , r_2 * block.1);
		snake::grow_snake(snake);
	}
}