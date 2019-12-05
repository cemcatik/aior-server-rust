use crate::errors::*;
use enigo::*;

pub struct Robot {
    enigo: Enigo,
    mouse_speed: f32,
    wheel_speed: f32,
}

pub enum MouseButton {
    Left,
    Right,
}

pub enum WheelDirection {
    Up,
    Down,
}

impl Robot {
    pub fn new(mouse_speed: f32, wheel_speed: f32) -> Robot {
        let enigo = Enigo::new();
        Robot {
            enigo,
            mouse_speed,
            wheel_speed,
        }
    }

    pub async fn mouse_move(&mut self, x: i32, y: i32) -> Result<()> {
        let x = (x as f32 * self.mouse_speed).round() as i32;
        let y = (y as f32 * self.mouse_speed).round() as i32;
        self.enigo.mouse_move_relative(x, y);
        Ok(())
    }

    pub async fn mouse_press(&mut self, button: MouseButton) -> Result<()> {
        let eb = enigo::MouseButton::from(button);
        self.enigo.mouse_down(eb);
        Ok(())
    }

    pub async fn mouse_release(&mut self, button: MouseButton) -> Result<()> {
        let eb = enigo::MouseButton::from(button);
        self.enigo.mouse_up(eb);
        Ok(())
    }

    pub async fn mouse_wheel(&mut self, dir: WheelDirection) -> Result<()> {
        let d = match dir {
            WheelDirection::Up => -1,
            WheelDirection::Down => 1,
        };
        let d = (d as f32 * self.wheel_speed).round() as i32;
        self.enigo.mouse_scroll_y(d);
        Ok(())
    }

    pub async fn keyboard_type_str(&mut self, letter: String) -> Result<()> {
        self.enigo.key_sequence(&letter);
        Ok(())
    }

    pub async fn keyboard_type_int(&mut self, key: u16) -> Result<()> {
        let key = enigo::Key::Raw(key);
        self.enigo.key_click(key);
        Ok(())
    }
}

impl From<MouseButton> for enigo::MouseButton {
    fn from(b: MouseButton) -> enigo::MouseButton {
        match b {
            MouseButton::Left => enigo::MouseButton::Left,
            MouseButton::Right => enigo::MouseButton::Right,
        }
    }
}
