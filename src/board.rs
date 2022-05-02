use raylib::prelude::*;

pub struct Board{
	pub x : u64,
	pub y : u64,
	pub color_1 : Color,
	pub color_2 : Color
}

pub fn build_board(x_ : u64, y_ : u64, color_1_ : Color, color_2_ : Color) -> Board
{
	Board
	{
		x : x_,
		y : y_,
		color_1 : color_1_,
		color_2 : color_2_,
	}
}

pub fn test(t : &mut RaylibDrawHandle, b : &Board)
{
	t.draw_text("SNAKE GAME!", 12, 12, 20, b.color_1);
}