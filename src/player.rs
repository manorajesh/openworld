use crate::Drawable;
use macroquad::prelude::*;

const DEFAULT_MOVE_SPEED: f32 = 3.0;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub move_speed: f32,
}

impl Drawable for Player {
    fn draw(&self, text_params: TextParams) {
        draw_text_ex("O", self.x, self.y, text_params);
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            x: 10.0,
            y: 10.0,
            move_speed: DEFAULT_MOVE_SPEED,
        }
    }
}

impl Player {
    pub fn process_input(&mut self) {
        let keys_down = get_keys_down();
        if keys_down.contains(&KeyCode::LeftShift) {
            self.move_speed = DEFAULT_MOVE_SPEED * 2.0;
        } else {
            self.move_speed = DEFAULT_MOVE_SPEED;
        }

        for key in keys_down {
            self.translate(key);
        }
    }

    fn translate(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::D => {
                self.x += self.move_speed;
            }
            KeyCode::A => {
                self.x -= self.move_speed;
            }
            KeyCode::S => {
                self.y += self.move_speed;
            }
            KeyCode::W => {
                self.y -= self.move_speed;
            }
            _ => {}
        }
    }
}
