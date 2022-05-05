use crate::board;
use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

pub struct Snake
{
	pub next  : (i32, i32),
	pub body  : Vec<(i32, i32)>,
	pub head_dir : String,
	pub color : Color,
	pub eye_color : Color,
	pub snake_eyes : ((i32, i32), (i32, i32)),
	pub snake_updated : bool,
}

pub fn build_snake(_color : Color, _eye_color : Color) -> Snake
{
	Snake
	{
		next : (0, 0),
		body : vec![],
		head_dir : "DOWN".to_string(),
		color : _color,
		snake_eyes : ((0, 0), (0, 0)),
		eye_color : _eye_color,
		snake_updated : false,
	}
}

pub fn init_snake(snake : &mut Snake, pos_init : (i32, i32),board : &board::Board, screen_size : (i32, i32))
{
	snake.body.push(pos_init); //head of the snake
	snake.next = (snake.body[0].0, snake.body[0].1 + (screen_size.1 / board.x));
}

pub fn create_snake(pos_init : (i32, i32), color : Color, eye_color : Color, board : &board::Board, screen_size : (i32, i32)) -> Snake
{
	let mut snake = build_snake(color, eye_color);
	init_snake(&mut snake, pos_init, board, screen_size);
	return snake;
}

pub fn draw_snake(h : &mut RaylibDrawHandle, snake : &mut Snake,block : (i32, i32))
{
	draw_snake_head(h, snake, block);

	for i in 1..snake.body.len() 
	{
		h.draw_rectangle(snake.body[i].0, snake.body[i].1, block.0, block.1, snake.color);
	}
}

pub fn draw_snake_head(h : &mut RaylibDrawHandle, snake : &mut Snake, block : (i32, i32))
{
	h.draw_rectangle(snake.body[0].0, snake.body[0].1, block.0, block.1, snake.color); //Draw head
	h.draw_circle(snake.snake_eyes.0.0, snake.snake_eyes.0.1, 5.0,snake.eye_color); //Draw Eyes
	h.draw_circle(snake.snake_eyes.1.0, snake.snake_eyes.1.1, 5.0,snake.eye_color);
}

pub fn update_snake(snake : &mut Snake, block : (i32, i32), screen_size : (i32, i32)) ->bool
{
	let snake_len = snake.body.len();
	let mut i = snake_len - 1;

	while i > 0
	{
		snake.body[i] = snake.body[i-1];
		i-=1;
	}

	snake.body[0] = snake.next; //Update the head


	for i in 1..snake_len //Test collision
	{
		if snake.body[0] == snake.body[i]
		{
			return false; //GAMEOVER
		}
	}

	match snake.head_dir.as_str() //Update the next pos and eyes
	{
		"UP"=>
		{
			snake.next = (snake.next.0, (snake.next.1 - block.1 + screen_size.1) % screen_size.1);
			snake.snake_eyes.0 = (snake.body[0].0 + (block.0/4), snake.body[0].1 + (block.0/4));
			snake.snake_eyes.1 = (snake.body[0].0 + (block.0/4) * 3, snake.body[0].1 + (block.0/4));
		}
		"DOWN"=>
		{
			snake.next = (snake.next.0, (snake.next.1 + block.1 + screen_size.1) % screen_size.1);
			snake.snake_eyes.0 = (snake.body[0].0 + (block.0/4), snake.body[0].1 + (block.0/4) * 3); 
			snake.snake_eyes.1 = (snake.body[0].0 + (block.0/4) * 3, snake.body[0].1 + (block.0/4) * 3);
		}
		"LEFT"=>
		{
			snake.next = ((snake.next.0 - block.0 + screen_size.0) % screen_size.0, snake.next.1);
			snake.snake_eyes.0 = (snake.body[0].0 + (block.0/4), snake.body[0].1 + (block.1/4));
			snake.snake_eyes.1 = (snake.body[0].0 + (block.0/4), snake.body[0].1 + (block.1/4) * 3);
		}
		"RIGHT"=>
		{
			snake.next = ((snake.next.0 + block.0 + screen_size.0) % screen_size.0, snake.next.1);
			snake.snake_eyes.0 = (snake.body[0].0 + (block.0/4) * 3, snake.body[0].1 + (block.1/4));
			snake.snake_eyes.1 = (snake.body[0].0 + (block.0/4) * 3, snake.body[0].1 + (block.1/4) * 3);
		},
		_=> snake.next = snake.next, //Default
	}
	snake.snake_updated = true;

	return true;
}

pub fn move_snake_dir(h : &mut RaylibDrawHandle, snake : &mut Snake)
{
	if snake.snake_updated
	{
		if h.is_key_pressed(KEY_UP) && snake.head_dir != "DOWN"
		{
			snake.head_dir = "UP".to_string();
			snake.snake_updated = false;
		}
		else if h.is_key_pressed(KEY_DOWN) && snake.head_dir != "UP"
		{		
			snake.head_dir = "DOWN".to_string();
			snake.snake_updated = false;
		}
		else if h.is_key_pressed(KEY_LEFT) && snake.head_dir != "RIGHT"
		{
			snake.head_dir = "LEFT".to_string();
			snake.snake_updated = false;
		}
		else if h.is_key_pressed(KEY_RIGHT) && snake.head_dir != "LEFT"
		{
			snake.head_dir = "RIGHT".to_string();
			snake.snake_updated = false;
		}
	}
}

pub fn grow_snake(snake : &mut Snake)
{
	snake.body.push(snake.body[snake.body.len()-1]);
}