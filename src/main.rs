use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
use raylib::core::text::*;

mod board;
mod snake;

fn main() 
{
    const SCREEN_SIZE: (i32, i32) = (800, 800);

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_SIZE.0, SCREEN_SIZE.1)
        .title("Snake")
        .build();

    let lime = Color::LIME;
    let light_lime = Color{r : lime.r, g : lime.g, b : lime.b, a : lime.a-200};
    let mut board = board::build_board(16, 16, Color::LIME, light_lime, SCREEN_SIZE);

    let mut snake = snake::build_snake(Color::GOLD, Color::WHITE); //Create the snake
    snake::init_snake(&mut snake, (SCREEN_SIZE.0/2, SCREEN_SIZE.1/2), &board, SCREEN_SIZE); //Init the snake

    let block : (i32, i32) = (SCREEN_SIZE.0/board.x, SCREEN_SIZE.1/board.y);

    let mut time : f32 = 0.0;

    let mut state = true; //Game state

    while !rl.window_should_close() 
    {
        let mut d = rl.begin_drawing(&thread);

        if !d.is_key_down(KEY_SPACE) //Press Space for pause
        {
            d.clear_background(Color::WHITE);

            board::draw_board(&mut d, &board, SCREEN_SIZE, block); //Draw the Game Board
            snake::draw_snake(&mut d, &mut snake, block); //Draw the Snake

            snake::move_snake_dir(&mut d, &mut snake); //Get Input and change dir
            
            time += d.get_frame_time();
            
            if time >= 0.2 && state == true
            {
                state = snake::update_snake(&mut snake, block, SCREEN_SIZE);
                board::update_board(&mut board, &mut snake, SCREEN_SIZE, block);
                time = 0.0
            }
            else if state == false
            {
                //SHow SCORE
                let mut score : String = "SCORE : ".to_string();
                score.push_str(&snake.body.len().to_string());
                let text_width : i32 = measure_text(&score, 40);

                d.draw_text(&score, SCREEN_SIZE.0/2 - (text_width/2), SCREEN_SIZE.1 / 2 - 20, 40,Color::BLACK);
                
                //Check if player want to replay
                
            }
        }
    }
}
