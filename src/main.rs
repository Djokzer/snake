use raylib::prelude::*;

mod board;

fn main() 
{
    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title("Snake")
        .build();

    let board = board::build_board(8, 8, Color::BLUE, Color::YELLOW);


    while !rl.window_should_close() 
    {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        //d.draw_text("SNAKE GAME!", 12, 12, 20, board.color_1);
        board::test(&mut d, &board);
    }
}
