use crate::errors::*;
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
    KeyboardStr { letter: String, state: u8 },

    #[serde(rename = "kib")]
    KeyboardInt { letter: u16, state: u8 },
}

impl Message {
    pub fn from_str(s: &str) -> Result<Message> {
        json5::from_str(s).map_err(|e| Error::from(e))
    }

    pub fn to_string(m: &Message) -> Result<String> {
        json5::to_string(m).map_err(|e| Error::from(e))
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
    MouseWheelUp = 60,
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
                r#"{{"type":"aioc","id":{}}}"#,
                AiocId::MouseLeftPress as u8
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
            let e = r#"{"type":"cs","sender":"server","status":"acceptUdpConnection","statusMessage":"os-version-arch"}"#;
            assert_eq!(e, j);
        }

        #[test]
        fn mouse_move() {
            let m = Message::MouseMove { x: 10, y: 135 };
            let j = Message::to_string(&m).unwrap();
            let e = r#"{"type":"mmb","x":10,"y":135}"#;
            assert_eq!(e, j);
        }

        #[test]
        fn keyboard_string() {
            let m = Message::KeyboardStr {
                letter: "cemcatik".to_string(),
                state: 13,
            };
            let j = Message::to_string(&m).unwrap();
            let e = r#"{"type":"ksb","letter":"cemcatik","state":13}"#;
            assert_eq!(e, j);
        }

        #[test]
        fn keyboard_int() {
            let m = Message::KeyboardInt {
                letter: 13,
                state: 14,
            };
            let j = Message::to_string(&m).unwrap();
            let e = r#"{"type":"kib","letter":13,"state":14}"#;
            assert_eq!(e, j);
        }
    }

    mod de {
        use super::super::*;

        #[test]
        fn ksb() {
            let expected = "C--e--x--backspace--m";
            let s = format!("{{type:'ksb',state:3,letter:'{}'}}", expected);
            match Message::from_str(&s) {
                Ok(Message::KeyboardStr { letter, state: _ }) => assert_eq!(expected, letter),
                _ => panic!(
                    "{} should have deserialized as KeyboardString({})",
                    s, expected
                ),
            }
        }

        #[test]
        fn ksb_letter_must_be_string() {
            let s = "{type:'ksb',state:3,letter:4}";
            match Message::from_str(&s) {
                Err(err) => assert_eq!(
                    err.to_string(),
                    "invalid type: integer `4`, expected a string"
                ),
                _ => panic!(
                    "should have failed to parse {} since 'letter' is not a string",
                    s
                ),
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
