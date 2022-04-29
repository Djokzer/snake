use raylib::prelude::*;

fn main() 
{
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Snake")
        .build();

    while !rl.window_should_close() 
    {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_text("SNAKE GAME!", 12, 12, 20, Color::WHITE);
    }
}
