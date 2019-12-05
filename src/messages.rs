use ascii::AsciiChar;
use serde::de::*;
use serde::*;
use serde_repr::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "aioc")]
    Aioc { id: AiocId },

    #[serde(rename = "cs")]
    ConnStatus {
        sender: String,
        status: String,

        #[serde(rename = "statusMessage")]
        message: String,
    },

    #[serde(rename = "mmb")]
    MouseMove { x: i32, y: i32 },

    #[serde(rename = "ksb")]
    KeyboardString {
        #[serde(deserialize_with = "split_keys")]
        letter: String,
        state: u8,
    },

    #[serde(rename = "kib")]
    KeyboardInt { letter: i32, state: u8 },
}

impl Message {
    pub fn from_str(s: &str) -> json5::Result<Message> {
        json5::from_str(s)
    }

    pub fn to_string(m: &Message) -> json5::Result<String> {
        json5::to_string(m)
    }
}

fn split_keys<'de, D>(deserializer: D) -> std::result::Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_string(KeyboardStringVisitor)
}

struct KeyboardStringVisitor;
impl<'de> Visitor<'de> for KeyboardStringVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string of characters separated with '--'")
    }

    fn visit_string<E>(self, value: String) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        fn char_or_special(s: &str) -> Vec<char> {
            match s {
                "backspace" => vec![AsciiChar::BackSpace.as_char()],
                "enter" => vec!['\n'],
                "space" => vec![' '],
                x => x.chars().collect::<Vec<_>>(),
            }
        }

        let parts: String = value.split("--").map(char_or_special).flatten().collect();
        return Ok(parts);
    }
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum AiocId {
    ConnectionReceived = 0,
    MouseLeftPress = 56,
    MouseLeftRelease = 57,
    MouseRightPress = 58,
    MouseRightRelease = 59,
    MouseWheelDown = 61,
}

#[cfg(test)]
mod tests {
    mod ser {
        use super::super::*;

        #[test]
        fn aioc() {
            let m = Message::Aioc {
                id: AiocId::MouseLeftPress,
            };
            let j = Message::to_string(&m).unwrap();
            let e = format!(
                "{{\"type\":\"aioc\",\"id\":{}}}",
                AiocId::MouseLeftPress as i32
            );
            assert_eq!(e, j);
        }

        #[test]
        fn conn_status() {
            let m = Message::ConnStatus {
                sender: "server".to_string(),
                status: "acceptUdpConnection".to_string(),
                message: "os-version-arch".to_string(),
            };
            let j = Message::to_string(&m).unwrap();
            let e = "{\"type\":\"cs\",\"sender\":\"server\",\"status\":\"acceptUdpConnection\",\"statusMessage\":\"os-version-arch\"}";
            assert_eq!(e, j);
        }

        #[test]
        fn mouse_move() {
            let m = Message::MouseMove { x: 10, y: 135 };
            let j = Message::to_string(&m).unwrap();
            let e = "{\"type\":\"mmb\",\"x\":10,\"y\":135}";
            assert_eq!(e, j);
        }

        #[test]
        fn keyboard_string() {
            let m = Message::KeyboardString {
                letter: "cemcatik".to_string(),
                state: 13,
            };
            let j = Message::to_string(&m).unwrap();
            let e = "{\"type\":\"ksb\",\"letter\":\"cemcatik\",\"state\":13}";
            assert_eq!(e, j);
        }

        #[test]
        fn keyboard_int() {
            let m = Message::KeyboardInt {
                letter: 13,
                state: 14,
            };
            let j = Message::to_string(&m).unwrap();
            let e = "{\"type\":\"kib\",\"letter\":13,\"state\":14}";
            assert_eq!(e, j);
        }
    }

    mod de {
        use super::super::*;

        fn assert_ksb(letter: &str, result: &str) {
            let s = format!("{{type:'ksb',state:3,letter:'{}'}}", letter);
            match Message::from_str(&s) {
                Ok(Message::KeyboardString { letter, state: _ }) => assert_eq!(result, letter),
                _ => panic!(
                    "{} should have deserialized as KeyboardString({})",
                    s, result
                ),
            }
        }

        #[test]
        fn ksb_single_char() {
            assert_ksb("F", "F");
        }

        #[test]
        fn ksb_two_chars() {
            assert_ksb("F--o", "Fo");
        }

        #[test]
        fn ksb_minus() {
            assert_ksb("-", "-");
        }

        #[test]
        fn ksb_minus_multi() {
            assert_ksb("F-----o", "F-o");
        }

        #[test]
        fn ksb_spec_space() {
            assert_ksb("C--e--m--space--C--a--t", "Cem Cat");
        }

        #[test]
        #[ignore]
        fn ksb_spec_backspace() {
            assert_ksb("C--e--x--backspace--m", "Cem");
        }

        #[test]
        fn ksb_letter_must_be_string() {
            let s = "{type:'ksb',state:3,letter:4}";
            match  Message::from_str(&s) {
                Err(err) => assert_eq!(err.description(), "invalid type: integer `4`, expected a string of characters separated with \'--\'"),
                _ => panic!("should have failed to parse {} since 'letter' is not a string", s),
            }
        }

        #[test]
        fn mouse_move() {
            let s = "{type:'mmb',x:509,y:531}";
            match Message::from_str(s) {
                Ok(Message::MouseMove { x, y }) => {
                    assert_eq!(509, x, "x");
                    assert_eq!(531, y, "y");
                }
                _ => panic!("{} should have deserialized as MouseMove(509, 531)", s),
            }
        }
    }
}
