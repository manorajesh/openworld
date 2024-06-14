mod player;

use macroquad::prelude::*;

pub trait Drawable {
    fn draw(&self, text_params: TextParams);
}

#[macroquad::main("openworld")]
async fn main() {
    let custom_font = load_ttf_font("assets/futura.ttf").await.unwrap();
    let mut player = player::Player::default();

    let camera = Camera2D {
        zoom: vec2(0.01, 0.01),
        target: vec2(0.0, 0.0),
        render_target: None,
        ..Default::default()
    };

    loop {
        clear_background(BLACK);

        set_camera(&camera);

        // update
        player.process_input();

        // draw
        let text_params = TextParams {
            font: Some(&custom_font),
            font_size: 30,
            font_scale: 1.0,
            color: WHITE,
            ..Default::default()
        };
        player.draw(text_params);

        set_default_camera();
        next_frame().await;
    }
}
