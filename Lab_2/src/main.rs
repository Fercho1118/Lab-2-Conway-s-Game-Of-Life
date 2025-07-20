use raylib::prelude::*;
mod framebuffer;
mod game_of_life;

use framebuffer::FrameBuffer;
use game_of_life::GameOfLife;

fn main() {
    let window_width = 800;
    let window_height = 600;
    let grid_width = 160;
    let grid_height = 120;

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Conway's Game of Life")
        .build();

    let mut framebuffer = FrameBuffer::new(grid_width, grid_height);
    let mut game = GameOfLife::new(grid_width as usize, grid_height as usize);

    game.setup_creative_patterns();

    let mut texture = rl.load_texture_from_image(&thread, &framebuffer.image).unwrap();

    let mut frame_count = 0;
    let frames_per_update = 10;

    while !rl.window_should_close() {
        if frame_count % frames_per_update == 0 {
            game.update();
        }
        frame_count += 1;

        game.render(&mut framebuffer);
        texture = rl.load_texture_from_image(&thread, &framebuffer.image).unwrap();


        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        let scale_x = window_width as f32 / grid_width as f32;
        let scale_y = window_height as f32 / grid_height as f32;
        let scale = scale_x.min(scale_y);

        let scaled_width = (grid_width as f32 * scale) as i32;
        let scaled_height = (grid_height as f32 * scale) as i32;
        let x_offset = (window_width - scaled_width) / 2;
        let y_offset = (window_height - scaled_height) / 2;

        d.draw_texture_ex(
            &texture,
            Vector2::new(x_offset as f32, y_offset as f32),
            0.0,
            scale,
            Color::WHITE,
        );

        d.draw_text("Conway's Game of Life", 10, 10, 20, Color::WHITE);
    }
}
