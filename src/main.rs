use raylib::prelude::*;

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
    let board = board::build_board(16, 16, Color::LIME, light_lime);

    let mut snake = snake::build_snake((SCREEN_SIZE.0/2, SCREEN_SIZE.1/2), &board, SCREEN_SIZE, Color::GOLD, Color::WHITE);

    let block : (i32, i32) = (SCREEN_SIZE.0/board.x, SCREEN_SIZE.1/board.y);

    let mut time : f32 = 0.0;
    while !rl.window_should_close() 
    {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        board::draw_board(&mut d, &board, SCREEN_SIZE); //Draw the Game Board
        snake::draw_snake(&mut d, &mut snake, &board,SCREEN_SIZE); //Draw the Snake

        time += d.get_frame_time();

        println!("{}", time);

        if time >= 0.25
        {
            snake::update_snake(&mut snake, block, SCREEN_SIZE);
            time = 0.0
        }
    }
}
