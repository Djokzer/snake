use raylib::prelude::*;
use Iterator;

pub struct Board{
	pub x : i32,
	pub y : i32,
	pub color_1 : Color,
	pub color_2 : Color,
}

pub fn build_board(x_ : i32, y_ : i32, color_1_ : Color, color_2_ : Color) -> Board
{
	Board
	{
		x : x_,
		y : y_,
		color_1 : color_1_,
		color_2 : color_2_,
	}
}

pub fn draw_board(h : &mut RaylibDrawHandle, b : &Board, size : (i32, i32))
{
	let mut c : Color;
	for x in (0..size.0).step_by((size.0/b.x) as usize)
	{
		for y in (0..size.1).step_by((size.1/b.y) as usize)
		{
			if ((x/(size.0/b.x)) + (y/(size.0/b.y))) % 2 == 0{c = b.color_1;}
			else{c = b.color_2;}
			h.draw_rectangle(x, y, size.0/b.x, size.1/b.y, c);
		}
	}
	//t.draw_text("SNAKE GAME!", 12, 12, 20, b.color_1);
}