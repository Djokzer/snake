use raylib::prelude::*;

mod board;

fn main() 
{
    const SIZE: (i32, i32) = (800, 800);

    let (mut rl, thread) = raylib::init()
        .size(SIZE.0, SIZE.1)
        .title("Snake")
        .build();

    let board = board::build_board(8, 8, Color::SKYBLUE, Color::LIGHTGRAY);


    while !rl.window_should_close() 
    {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        board::draw_board(&mut d, &board, SIZE);
    }
}
