use raylib::prelude::*;

mod board;

fn main() 
{
    const SIZE: (i32, i32) = (800, 800);

    let (mut rl, thread) = raylib::init()
        .size(SIZE.0, SIZE.1)
        .title("Snake")
        .build();

    let lime = Color::LIME;
    let light_lime = Color{r : lime.r, g : lime.g, b : lime.b, a : lime.a-200};
    let board = board::build_board(16, 16, Color::LIME, light_lime);


    while !rl.window_should_close() 
    {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        board::draw_board(&mut d, &board, SIZE);
    }
}
