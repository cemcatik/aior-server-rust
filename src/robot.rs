use crate::messages::*;
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

    pub fn handle(&mut self, m: Message) {
        match m {
            Message::MouseMove { x, y } => self.mouse_move(x, y),
            Message::Aioc {
                id: AiocId::MouseLeftPress,
            } => self.mouse_press(MouseButton::Left),
            Message::Aioc {
                id: AiocId::MouseLeftRelease,
            } => self.mouse_release(MouseButton::Left),
            Message::Aioc {
                id: AiocId::MouseRightPress,
            } => self.mouse_press(MouseButton::Right),
            Message::Aioc {
                id: AiocId::MouseRightRelease,
            } => self.mouse_release(MouseButton::Right),
            Message::Aioc {
                id: AiocId::MouseWheelUp,
            } => self.mouse_wheel(WheelDirection::Up),
            Message::Aioc {
                id: AiocId::MouseWheelDown,
            } => self.mouse_wheel(WheelDirection::Down),
            Message::KeyboardStr { letter, .. } => self.keyboard_type_str(letter),
            Message::KeyboardInt { letter, .. } => self.keyboard_type_int(letter),
            _ => {
                println!("maybe next time");
            }
        }
    }

    pub fn mouse_move(&mut self, x: i32, y: i32) {
        let x = x * self.mouse_speed.round() as i32;
        let y = y * self.mouse_speed.round() as i32;
        self.enigo.mouse_move_relative(x, y);
    }

    fn mouse_press(&mut self, button: MouseButton) {
        let eb = enigo::MouseButton::from(button);
        self.enigo.mouse_down(eb);
    }

    fn mouse_release(&mut self, button: MouseButton) {
        let eb = enigo::MouseButton::from(button);
        self.enigo.mouse_up(eb);
    }

    fn mouse_wheel(&mut self, dir: WheelDirection) {
        let d = match dir {
            WheelDirection::Up => -1,
            WheelDirection::Down => 1,
        };
        let d = d * self.wheel_speed.round() as i32;
        self.enigo.mouse_scroll_y(d);
    }

    fn process_keys<F: FnMut(Key)>(letter: String, callback: &mut F) {
        letter.split("--").for_each(|l| match l {
            "backspace" => callback(enigo::Key::Backspace),
            "enter" => callback(enigo::Key::Return),
            "space" => callback(enigo::Key::Space),
            x => x.chars().for_each(|c| callback(enigo::Key::Layout(c))),
        });
    }

    fn keyboard_type_str(&mut self, letter: String) {
        Robot::process_keys(letter, &mut |k| self.enigo.key_click(k));
    }

    fn keyboard_type_int(&mut self, key: u16) {
        let key = enigo::Key::Raw(key);
        self.enigo.key_click(key);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_process_keys(letter: &str, expected: Vec<enigo::Key>) {
        let mut actual = vec![];
        Robot::process_keys(String::from(letter), &mut |k| actual.push(k));
        assert_eq!(&expected, &actual);
    }

    #[test]
    fn ksb_single_char() {
        assert_process_keys("F", vec![enigo::Key::Layout('F')]);
    }

    #[test]
    fn ksb_two_chars() {
        assert_process_keys(
            "F--o",
            vec![enigo::Key::Layout('F'), enigo::Key::Layout('o')],
        );
    }

    #[test]
    fn ksb_minus() {
        assert_process_keys("-", vec![enigo::Key::Layout('-')]);
    }

    #[test]
    fn ksb_minus_multi() {
        "F-----o".split("--").for_each(|k| println!("{}", k));
        assert_process_keys(
            "F-----o",
            vec![
                enigo::Key::Layout('F'),
                enigo::Key::Layout('-'),
                enigo::Key::Layout('o'),
            ],
        );
    }

    #[test]
    fn ksb_spec_space() {
        assert_process_keys(
            "C--e--m--space--C--a--t",
            vec![
                enigo::Key::Layout('C'),
                enigo::Key::Layout('e'),
                enigo::Key::Layout('m'),
                enigo::Key::Space,
                enigo::Key::Layout('C'),
                enigo::Key::Layout('a'),
                enigo::Key::Layout('t'),
            ],
        );
    }

    #[test]
    fn ksb_spec_backspace() {
        assert_process_keys(
            "C--e--x--backspace--m",
            vec![
                enigo::Key::Layout('C'),
                enigo::Key::Layout('e'),
                enigo::Key::Layout('x'),
                enigo::Key::Backspace,
                enigo::Key::Layout('m'),
            ],
        );
    }
}
