use uuid::Uuid;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub enum State {
    Open,
    Done
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct TodoItem {
    text: String,
    id: String,
    state: State
}

impl TodoItem {
    pub fn new(text: String) -> TodoItem {
        TodoItem {
            text: text,
            id: Uuid::new_v4().to_string(),
            state: State::Open
        }
    }

    pub fn get_id(self) -> String {
        self.id
    }
}

#[derive(RustcDecodable)]
pub struct TodoPartial {
    pub text: String,
}
