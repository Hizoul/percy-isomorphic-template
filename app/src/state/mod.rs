use serde::{Deserialize, Serialize};
use serde_json;
use std::cell::Cell;
use std::rc::Rc;

mod msg;
pub use self::msg::Msg;

#[derive(Serialize, Deserialize)]
pub struct State {
    path: String
}

impl State {
    pub fn new() -> State {
        State {
            path: "/".to_string()
        }
    }

    pub fn from_json(state_json: &str) -> State {
        serde_json::from_str(state_json).unwrap()
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    pub fn msg(&mut self, msg: &Msg) {
        match msg {
            Msg::SetPath(path) => self.set_path(path.to_string())
        };
    }
    pub fn path(&self) -> &str {
        &self.path
    }
    fn set_path(&mut self, path: String) {
        self.path = path;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn serialize_deserialize() {
        let mut state = State::from_json(r#"{path":"/"}"#);
        assert_eq!(state.path(), "/");
        state.set_path("/test".to_owned());
        assert_eq!(state.path(), "/test");
        assert_eq!(&state.to_json(), r#"{path":"/test"}"#);
    }
}
