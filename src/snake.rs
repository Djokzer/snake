use crate::board;
use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

pub struct Snake
{
	pub next  : (i32, i32),
	pub head  : (i32, i32),
	pub body  : Vec<(i32, i32)>,
	pub head_dir : String,
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
		head_dir : "DOWN".to_string(),
		color : _color,
		eye_color : _eye_color,
	}
}

pub fn draw_snake(h : &mut RaylibDrawHandle, snake : &mut Snake, b : &board::Board, block : (i32, i32))
{
	draw_snake_head(h, snake, block);

}

pub fn draw_snake_head(h : &mut RaylibDrawHandle, snake : &mut Snake, block : (i32, i32))
{
	h.draw_rectangle(snake.head.0, snake.head.1, block.0, block.1, snake.color); //Draw head
	
	let eyes_pos : (i32, i32);
	let eyes_pos_2 : (i32, i32);

	match snake.head_dir.as_str()
	{
		"UP"=> 
		{
			eyes_pos = (snake.head.0 + (block.0/4), snake.head.1 + (block.0/4)); 
			eyes_pos_2 = (snake.head.0 + (block.0/4) * 3, snake.head.1 + (block.0/4));
		},
		"DOWN"=>
		{
			eyes_pos = (snake.head.0 + (block.0/4), snake.head.1 + (block.0/4) * 3); 
			eyes_pos_2 = (snake.head.0 + (block.0/4) * 3, snake.head.1 + (block.0/4) * 3);
		},
		"LEFT"=>
		{
			eyes_pos = (snake.head.0 + (block.0/4), snake.head.1 + (block.1/4));
			eyes_pos_2 = (snake.head.0 + (block.0/4), snake.head.1 + (block.1/4) * 3);
		},
		"RIGHT"=>
		{
			eyes_pos = (snake.head.0 + (block.0/4) * 3, snake.head.1 + (block.1/4));
			eyes_pos_2 = (snake.head.0 + (block.0/4) * 3, snake.head.1 + (block.1/4) * 3);
		},
		_=>
		{
			eyes_pos = (snake.head.0 + (block.0/4), 0); 
			eyes_pos_2 = (0,0)
		},
	}
	h.draw_circle(eyes_pos.0, eyes_pos.1, 5.0, snake.eye_color);
	h.draw_circle(eyes_pos_2.0, eyes_pos_2.1, 5.0, snake.eye_color);
}

pub fn update_snake(snake : &mut Snake, block : (i32, i32), screen_size : (i32, i32))
{
	snake.head = snake.next;

	match snake.head_dir.as_str()
	{
		"UP"=>snake.next = (snake.next.0, (snake.next.1 - block.1 + screen_size.1) % screen_size.1),
		"DOWN"=>snake.next = (snake.next.0, (snake.next.1 + block.1 + screen_size.1) % screen_size.1),
		"LEFT"=>snake.next = ((snake.next.0 - block.0 + screen_size.0) % screen_size.0, snake.next.1),
		"RIGHT"=>snake.next = ((snake.next.0 + block.0 + screen_size.0) % screen_size.0, snake.next.1),
		_=> snake.next = snake.next, //Default
	}
}

pub fn move_snake_dir(h : &mut RaylibDrawHandle, snake : &mut Snake)
{
	if h.is_key_pressed(KEY_UP) && snake.head_dir != "DOWN"
	{
		snake.head_dir = "UP".to_string();
	}
	else if h.is_key_pressed(KEY_DOWN) && snake.head_dir != "UP"
	{		
		snake.head_dir = "DOWN".to_string();
	}
	else if h.is_key_pressed(KEY_LEFT) && snake.head_dir != "RIGHT"
	{
		snake.head_dir = "LEFT".to_string();
	}
	else if h.is_key_pressed(KEY_RIGHT) && snake.head_dir != "LEFT"
	{
		snake.head_dir = "RIGHT".to_string();
	}
}